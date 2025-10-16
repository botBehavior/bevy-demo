use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
#[cfg(not(target_arch = "wasm32"))]
use bevy::window::WindowResolution;
use rand::prelude::*;
use std::time::Duration;

const PLAYER_SPEED: f32 = 900.0;
const PLAYER_RADIUS: f32 = 14.0;
const PLAYER_MAX_HEALTH: i32 = 3;
const PLAYER_COLLISION_DAMAGE: i32 = 1;
const TRAIL_LIFETIME: f32 = 2.6;
const TRAIL_SPAWN_INTERVAL: f32 = 0.028;
const TRAIL_HIT_RADIUS: f32 = 16.0;
const ENEMY_BASE_SPEED: f32 = 220.0;
const ENEMY_SPEED_INCREMENT: f32 = 8.0;
const ENEMY_SPAWN_INTERVAL_START: f32 = 1.2;
const ENEMY_SPAWN_ACCELERATION: f32 = 0.92;
const ENEMY_SIZE: Vec2 = Vec2::new(36.0, 36.0);
const COMBO_WINDOW: f32 = 1.2;
const BASE_SCORE: u32 = 10;
const ARENA_BOUNDS: Vec2 = Vec2::new(960.0, 720.0);

fn main() {
    #[cfg(target_arch = "wasm32")]
    init_wasm_panic_hooks();

    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(RunState::default())
        .insert_resource(PointerTarget::default())
        .insert_resource(Score::default())
        .insert_resource(Combo::default())
        .insert_resource(PlayerHealth::default())
        .insert_resource(EnemySpawnTimer::default())
        .insert_resource(TrailSpawnTimer::default())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(primary_window()),
                    ..Default::default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    ..Default::default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_pointer_target,
                move_player,
                spawn_trail_segments,
                update_trail_segments,
                spawn_enemies,
                move_enemies,
                handle_trail_collisions,
                handle_player_collisions,
                tick_combo,
                update_ui,
                handle_restart,
            ),
        );

    #[cfg(target_arch = "wasm32")]
    {
        use bevy::winit::WinitSettings;
        app.insert_resource(WinitSettings::game());
    }

    app.run();
}

#[cfg(target_arch = "wasm32")]
fn init_wasm_panic_hooks() {
    console_error_panic_hook::set_once();
}

#[cfg(target_arch = "wasm32")]
fn primary_window() -> Window {
    Window {
        title: "Threadweaver".to_string(),
        canvas: Some("#bevy-canvas".into()),
        fit_canvas_to_parent: true,
        prevent_default_event_handling: true,
        ..Default::default()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn primary_window() -> Window {
    Window {
        title: "Threadweaver".to_string(),
        resolution: WindowResolution::new(ARENA_BOUNDS.x, ARENA_BOUNDS.y),
        resizable: true,
        ..Default::default()
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy {
    speed: f32,
}

#[derive(Component)]
struct TrailSegment {
    remaining: f32,
}

#[derive(Resource)]
struct PlayerHealth {
    current: i32,
    max: i32,
}

impl Default for PlayerHealth {
    fn default() -> Self {
        Self {
            current: PLAYER_MAX_HEALTH,
            max: PLAYER_MAX_HEALTH,
        }
    }
}

impl PlayerHealth {
    fn reset(&mut self) {
        self.current = self.max;
    }

    fn apply_damage(&mut self, amount: i32) {
        self.current = (self.current - amount).clamp(0, self.max);
    }
}

#[derive(Resource)]
struct RunState {
    active: bool,
}

impl Default for RunState {
    fn default() -> Self {
        Self { active: true }
    }
}

#[derive(Resource, Default)]
struct PointerTarget(Option<Vec2>);

#[derive(Resource, Default)]
struct Score {
    current: u32,
    best: u32,
}

#[derive(Resource)]
struct Combo {
    streak: u32,
    multiplier: f32,
    timer: Timer,
}

impl Default for Combo {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(COMBO_WINDOW, TimerMode::Once);
        timer.pause();
        Self {
            streak: 0,
            multiplier: 1.0,
            timer,
        }
    }
}

impl Combo {
    fn register_kill(&mut self) -> u32 {
        if self.timer.paused() {
            self.timer.unpause();
        }
        self.timer.reset();
        self.streak += 1;
        self.multiplier = 1.0 + (self.streak.saturating_sub(1) as f32) * 0.5;
        (BASE_SCORE as f32 * self.multiplier).round() as u32
    }

    fn reset(&mut self) {
        self.streak = 0;
        self.multiplier = 1.0;
        self.timer.pause();
        self.timer.reset();
    }
}

#[derive(Resource)]
struct EnemySpawnTimer {
    timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_INTERVAL_START, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
struct TrailSpawnTimer(Timer);

impl Default for TrailSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            TRAIL_SPAWN_INTERVAL,
            TimerMode::Repeating,
        ))
    }
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, player_health: Res<PlayerHealth>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.2).with_scale(Vec3::splat(1.2)),
            sprite: Sprite {
                color: Color::srgb(0.6, 0.9, 1.0),
                custom_size: Some(Vec2::splat(28.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Player,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                format!(
                    "Score: 0 | Best: 0 | Combo x1.0 | Health: {}/{}\n",
                    player_health.current, player_health.max
                ),
                TextStyle {
                    font_size: 22.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            TextSection::new(
                "Status: Running\nPress SPACE to restart after a crash.",
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(0.6, 0.6, 0.6),
                    ..Default::default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(16.0),
            ..Default::default()
        }),
        ScoreText,
    ));
}

fn update_pointer_target(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut target: ResMut<PointerTarget>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok((camera, transform)) = camera_query.get_single() else {
        return;
    };

    if let Some(ray) = camera.viewport_to_world(transform, cursor_position) {
        target.0 = Some(ray.origin.truncate());
    }
}

fn move_player(
    time: Res<Time>,
    run_state: Res<RunState>,
    mut query: Query<&mut Transform, With<Player>>,
    target: Res<PointerTarget>,
) {
    if !run_state.active {
        return;
    }

    let Some(target_position) = target.0 else {
        return;
    };

    let mut transform = query.single_mut();
    let current = transform.translation.truncate();
    let delta = target_position - current;
    let distance = delta.length();

    if distance <= f32::EPSILON {
        return;
    }

    let step = PLAYER_SPEED * time.delta_seconds();
    if step >= distance {
        transform.translation.x = target_position.x;
        transform.translation.y = target_position.y;
    } else {
        let direction = delta / distance;
        transform.translation.x += direction.x * step;
        transform.translation.y += direction.y * step;
    }

    clamp_to_bounds(&mut transform.translation);
}

fn clamp_to_bounds(translation: &mut Vec3) {
    let half_width = ARENA_BOUNDS.x * 0.5;
    let half_height = ARENA_BOUNDS.y * 0.5;
    translation.x = translation.x.clamp(-half_width, half_width);
    translation.y = translation.y.clamp(-half_height, half_height);
}

fn spawn_trail_segments(
    mut commands: Commands,
    time: Res<Time>,
    run_state: Res<RunState>,
    mut timer: ResMut<TrailSpawnTimer>,
    player_query: Query<&Transform, With<Player>>,
) {
    if !run_state.active {
        return;
    }

    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }

    timer.0.reset();

    let transform = player_query.single();
    let position = transform.translation;

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(position.x, position.y, 0.1),
            sprite: Sprite {
                color: Color::srgba(0.3, 0.9, 1.0, 0.85),
                custom_size: Some(Vec2::splat(18.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        TrailSegment {
            remaining: TRAIL_LIFETIME,
        },
    ));
}

fn update_trail_segments(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TrailSegment, &mut Sprite, &mut Transform)>,
) {
    for (entity, mut segment, mut sprite, mut transform) in &mut query {
        segment.remaining -= time.delta_seconds();
        if segment.remaining <= 0.0 {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        let life_ratio = (segment.remaining / TRAIL_LIFETIME).clamp(0.0, 1.0);
        sprite.color.set_alpha(0.1 + 0.75 * life_ratio);
        let scale = 0.5 + life_ratio * 0.5;
        transform.scale = Vec3::splat(scale);
    }
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    run_state: Res<RunState>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    score: Res<Score>,
) {
    if !run_state.active {
        return;
    }

    if !spawn_timer.timer.tick(time.delta()).just_finished() {
        return;
    }

    let duration = spawn_timer.timer.duration().as_secs_f32();
    let new_duration = (duration * ENEMY_SPAWN_ACCELERATION).max(0.35);
    spawn_timer
        .timer
        .set_duration(Duration::from_secs_f32(new_duration));
    spawn_timer.timer.reset();

    let mut rng = thread_rng();
    let spawn_edge = rng.gen_range(0..4);
    let mut position = Vec2::ZERO;
    let half_width = ARENA_BOUNDS.x * 0.5;
    let half_height = ARENA_BOUNDS.y * 0.5;

    match spawn_edge {
        0 => {
            position.x = -half_width;
            position.y = rng.gen_range(-half_height..half_height);
        }
        1 => {
            position.x = half_width;
            position.y = rng.gen_range(-half_height..half_height);
        }
        2 => {
            position.y = -half_height;
            position.x = rng.gen_range(-half_width..half_width);
        }
        _ => {
            position.y = half_height;
            position.x = rng.gen_range(-half_width..half_width);
        }
    }

    let speed_bonus = ENEMY_SPEED_INCREMENT * score.current as f32 / 200.0;
    let enemy_speed = ENEMY_BASE_SPEED + speed_bonus;

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(position.x, position.y, 0.15),
            sprite: Sprite {
                color: Color::srgba(1.0, 0.45, 0.55, 0.95),
                custom_size: Some(ENEMY_SIZE),
                ..Default::default()
            },
            ..Default::default()
        },
        Enemy { speed: enemy_speed },
    ));
}

fn move_enemies(
    time: Res<Time>,
    run_state: Res<RunState>,
    player_query: Query<&Transform, With<Player>>,
    mut enemies: Query<(&Enemy, &mut Transform)>,
) {
    if !run_state.active {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    for (enemy, mut transform) in &mut enemies {
        let direction = player_pos - transform.translation.truncate();
        let distance = direction.length();
        if distance > f32::EPSILON {
            let normalized = direction / distance;
            transform.translation.x += normalized.x * enemy.speed * time.delta_seconds();
            transform.translation.y += normalized.y * enemy.speed * time.delta_seconds();
        }
    }
}

fn handle_trail_collisions(
    mut commands: Commands,
    run_state: Res<RunState>,
    mut score: ResMut<Score>,
    mut combo: ResMut<Combo>,
    enemies: Query<(Entity, &Transform, &Enemy)>,
    trail: Query<&Transform, With<TrailSegment>>,
) {
    if !run_state.active {
        return;
    }

    let mut despawn = Vec::new();

    for (enemy_entity, enemy_transform, _enemy) in &enemies {
        let enemy_pos = enemy_transform.translation.truncate();
        let mut destroyed = false;
        for segment_transform in &trail {
            let diff = enemy_pos - segment_transform.translation.truncate();
            if diff.length_squared() <= TRAIL_HIT_RADIUS * TRAIL_HIT_RADIUS {
                destroyed = true;
                break;
            }
        }

        if destroyed {
            score.current += combo.register_kill();
            despawn.push(enemy_entity);
        }
    }

    for entity in despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_player_collisions(
    mut commands: Commands,
    mut run_state: ResMut<RunState>,
    mut score: ResMut<Score>,
    mut combo: ResMut<Combo>,
    mut health: ResMut<PlayerHealth>,
    player_query: Query<&Transform, With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    if !run_state.active {
        return;
    }

    let player_transform = player_query.single();
    let player_pos = player_transform.translation.truncate();

    for (entity, transform) in &enemies {
        let diff = transform.translation.truncate() - player_pos;
        if diff.length_squared() <= PLAYER_RADIUS * PLAYER_RADIUS {
            health.apply_damage(PLAYER_COLLISION_DAMAGE);
            commands.entity(entity).despawn_recursive();
            if health.current <= 0 {
                health.current = 0;
                run_state.active = false;
                score.best = score.best.max(score.current);
                combo.reset();
            }
            break;
        }
    }
}

fn tick_combo(time: Res<Time>, mut combo: ResMut<Combo>, run_state: Res<RunState>) {
    if !run_state.active {
        return;
    }

    if combo.timer.paused() {
        return;
    }

    combo.timer.tick(time.delta());
    if combo.timer.finished() {
        combo.reset();
    }
}

fn update_ui(
    run_state: Res<RunState>,
    score: Res<Score>,
    combo: Res<Combo>,
    health: Res<PlayerHealth>,
    mut texts: Query<&mut Text, With<ScoreText>>,
) {
    let mut text = texts.single_mut();
    text.sections[0].value = format!(
        "Score: {} | Best: {} | Combo x{:.1} | Health: {}/{}\n",
        score.current, score.best, combo.multiplier, health.current, health.max
    );

    if run_state.active {
        text.sections[1].value =
            "Status: Running\nPress SPACE to restart after a crash.".to_string();
    } else {
        text.sections[1].value = "Status: Down! Press SPACE to respawn.".to_string();
    }
}

fn handle_restart(
    keys: Res<ButtonInput<KeyCode>>,
    mut run_state: ResMut<RunState>,
    mut score: ResMut<Score>,
    mut combo: ResMut<Combo>,
    mut pointer: ResMut<PointerTarget>,
    mut enemy_spawn: ResMut<EnemySpawnTimer>,
    mut trail_timer: ResMut<TrailSpawnTimer>,
    mut health: ResMut<PlayerHealth>,
    mut player_query: Query<&mut Transform, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
    trail_segments: Query<Entity, With<TrailSegment>>,
    mut commands: Commands,
) {
    if run_state.active {
        return;
    }

    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    run_state.active = true;
    score.current = 0;
    combo.reset();
    pointer.0 = None;
    enemy_spawn.timer = Timer::from_seconds(ENEMY_SPAWN_INTERVAL_START, TimerMode::Repeating);
    trail_timer.0 = Timer::from_seconds(TRAIL_SPAWN_INTERVAL, TimerMode::Repeating);
    health.reset();

    for entity in enemies.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in trail_segments.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let mut player_transform = player_query.single_mut();
    player_transform.translation = Vec3::new(0.0, 0.0, player_transform.translation.z);
}
