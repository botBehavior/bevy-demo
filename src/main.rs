use bevy::input::mouse::MouseMotion;
#[cfg(target_arch = "wasm32")]
use bevy::log::LogPlugin;
use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::window::WindowResolution;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use rand::prelude::*;
use std::time::Duration;

// Physics engine
use avian2d::prelude::*;

// V2: Rebalanced Core Constants
const PLAYER_SPEED: f32 = 950.0; // Was 900 - more responsive
const PLAYER_RADIUS: f32 = 14.0;
const PLAYER_MAX_HEALTH: u32 = 4; // Was 5 - faster deaths
const PLAYER_COLLISION_DAMAGE: u32 = 1;
const TRAIL_LIFETIME: f32 = 2.6;
const TRAIL_SPAWN_INTERVAL: f32 = 0.028;
const TRAIL_HIT_RADIUS: f32 = 16.0;
const ENEMY_BASE_SPEED: f32 = 180.0; // Was 220 - less overwhelming
const ENEMY_SPEED_INCREMENT: f32 = 8.0;
const ENEMY_SPAWN_INTERVAL_START: f32 = 2.0; // Was 1.2 - more breathing room
const ENEMY_SPAWN_ACCELERATION: f32 = 0.92;
const ENEMY_SIZE: Vec2 = Vec2::new(36.0, 36.0);
const COMBO_WINDOW: f32 = 1.0; // Was 1.2 - tighter timing
const COMBO_MULTIPLIER_STEP: f32 = 0.5;
const BASE_SCORE: u32 = 10;
const ARENA_BOUNDS: Vec2 = Vec2::new(1024.0, 768.0);
const ENEMY_BASE_HEALTH: u32 = 3; // Was 4 - easier early game
const TRAIL_BASE_DAMAGE: u32 = 3; // Was 1 - feel powerful early
const DAMAGE_POWER_BONUS: f32 = 0.5;
const SHIELD_DURATION: f32 = 4.0; // Was 10 - tactical not invincible
const POWER_UP_LIFETIME: f32 = 12.0;
const POWER_UP_DROP_CHANCE: f32 = 0.15; // Was 0.35 - rare = special
const POWER_UP_HEART_WEIGHT: f32 = 0.35; // Rebalanced for 4 types
const POWER_UP_SHIELD_WEIGHT: f32 = 0.25;
const POWER_UP_DAMAGE_WEIGHT: f32 = 0.25;
const POWER_UP_ACCURACY_WEIGHT: f32 = 0.15; // V2.5: New accuracy power-up

// Game Feel Constants
const SCREEN_SHAKE_DECAY: f32 = 3.0;
const ENEMY_KNOCKBACK: f32 = 250.0;
const PLAYER_KNOCKBACK_STRENGTH: f32 = 200.0;
const HIT_FREEZE_DURATION: f32 = 0.04;
const PLAYER_ACCELERATION: f32 = 0.12;
const PLAYER_DECELERATION: f32 = 0.25;
const ENEMY_TURN_SPEED: f32 = 0.18;

// V2: Infinite Space Constants
const CAMERA_SMOOTHING: f32 = 0.30; // V2.6 FIX: Was 0.08 - way too slow!
const ARENA_SIZE: f32 = 5000.0;
const ENEMY_SPAWN_DISTANCE: f32 = 600.0;

// V2: Wave Weapon Constants
const WAVE_COOLDOWN: f32 = 0.35;
const WAVE_PROJECTILE_COUNT: u32 = 5;
const WAVE_SPREAD_ANGLE: f32 = 0.4;
const WAVE_SPEED: f32 = 800.0;
const WAVE_LIFETIME: f32 = 1.5;
const WAVE_DAMAGE: u32 = 2;

// V2: Rebalanced for Power Fantasy
const PLAYER_START_HEALTH: u32 = 4; // Was 5
const PLAYER_BOOSTED_SPEED: f32 = 950.0; // Was 900
const TRAIL_START_DAMAGE: u32 = 3; // Was 1
const ENEMY_START_SPEED: f32 = 180.0; // Was 220
const COMBO_TIGHTER_WINDOW: f32 = 1.0; // Was 1.2
const SHIELD_TACTICAL_DURATION: f32 = 4.0; // Was 10
const POWER_UP_RARE_CHANCE: f32 = 0.15; // Was 0.35

fn main() {
    #[cfg(target_arch = "wasm32")]
    init_wasm_panic_hooks();

    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(RunState::default())
        .insert_resource(PointerTarget::default())
        .insert_resource(PlayerHealth::default())
        .insert_resource(PlayerCombatStats::default())
        .insert_resource(ShieldState::default())
        .insert_resource(CursorLockState::default())
        .insert_resource(Score::default())
        .insert_resource(Combo::default())
        .insert_resource(EnemySpawnTimer::default())
        .insert_resource(TrailSpawnTimer::default())
        .add_plugins({
            #[cfg(target_arch = "wasm32")]
            {
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(primary_window()),
                        ..Default::default()
                    })
                    .disable::<LogPlugin>()
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(primary_window()),
                    ..Default::default()
                })
            }
        })
        // Physics engine (Avian2D)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::ZERO)) // Top-down game, no gravity
        .add_systems(Startup, setup)
        // Core gameplay systems
        .add_systems(
            Update,
            (
                update_pointer_target,
                tick_shield_state,
                move_player,
                spawn_trail_segments,
                update_trail_segments,
                spawn_enemies,
                move_enemies,
                handle_trail_collisions,
                handle_player_collisions,
            ),
        )
        // UI and management systems
        .add_systems(
            Update,
            (
                handle_power_up_pickups,
                tick_power_up_lifetimes,
                tick_combo,
                update_ui,
                handle_restart,
                handle_pause_toggle,
                enforce_cursor_lock,
            ),
        )
        // V2: New weapon and camera systems
        .add_systems(
            Update,
            (
                camera_follow_player,
                update_background_tiles, // V2.5: Infinite background
                toggle_weapon,
                spawn_wave_projectiles,
                update_wave_projectiles,
                handle_wave_collisions,
            ),
        )
        // Game feel systems
        .add_systems(
            Update,
            (
                update_screen_shake,
                apply_knockback,
                tick_hit_freeze,
                despawn_finished_effects,
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
        resolution: WindowResolution::new(1024.0, 768.0),
        resizable: true,
        ..Default::default()
    }
}

// V2: Weapon Types
#[derive(Clone, Copy, PartialEq)]
enum WeaponType {
    Trail,
    Wave,
}

#[derive(Component)]
struct Player {
    weapon_type: WeaponType,
    wave_cooldown: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            weapon_type: WeaponType::Trail,
            wave_cooldown: 0.0,
        }
    }
}

#[derive(Component)]
struct Enemy {
    speed: f32,
}

#[derive(Component)]
struct EnemyHealth {
    current: f32,
}

#[derive(Component)]
struct TrailSegment {
    remaining: f32,
}

// Game Feel Components
#[derive(Component)]
struct ScreenShake {
    trauma: f32, // 0.0 to 1.0
}

impl Default for ScreenShake {
    fn default() -> Self {
        Self { trauma: 0.0 }
    }
}

#[derive(Component, Default)]
struct Knockback {
    velocity: Vec2,
}

#[derive(Component)]
struct PlayerVelocity {
    current: Vec2,
}

#[derive(Component)]
struct EnemyVelocity {
    current: Vec2,
}

#[derive(Resource)]
struct RunState {
    active: bool,
    paused: bool,
}

impl RunState {
    fn is_running(&self) -> bool {
        self.active && !self.paused
    }
}

impl Default for RunState {
    fn default() -> Self {
        Self {
            active: true,
            paused: false,
        }
    }
}

#[derive(Resource, Default)]
struct PointerTarget {
    position: Vec2,
}

impl PointerTarget {
    fn reset(&mut self) {
        self.position = Vec2::ZERO;
    }
}

#[derive(Resource, Default)]
struct CursorLockState {
    locked: bool,
}

#[derive(Resource)]
struct PlayerHealth {
    current: u32,
    max: u32,
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
    fn heal(&mut self, amount: u32) {
        self.current = (self.current + amount).min(self.max);
    }

    fn reset(&mut self) {
        self.current = self.max;
    }

    fn apply_damage(&mut self, amount: u32) {
        if amount >= self.current {
            self.current = 0;
        } else {
            self.current -= amount;
        }
    }
}

#[derive(Resource, Default)]
struct ShieldState {
    timer: Option<Timer>,
}

impl ShieldState {
    fn activate(&mut self) {
        let mut timer = Timer::from_seconds(SHIELD_DURATION, TimerMode::Once);
        timer.unpause();
        self.timer = Some(timer);
    }

    fn is_active(&self) -> bool {
        matches!(self.timer.as_ref(), Some(timer) if !timer.finished())
    }

    fn clear(&mut self) {
        self.timer = None;
    }

    fn remaining_seconds(&self) -> Option<f32> {
        self.timer
            .as_ref()
            .and_then(|timer| (!timer.finished()).then(|| timer.remaining_secs()))
    }
}

#[derive(Resource)]
struct PlayerCombatStats {
    base_trail_damage: f32,
    bonus_multiplier: f32,
    accuracy_stacks: u32, // V2.5: Accuracy power-up tracking
}

impl Default for PlayerCombatStats {
    fn default() -> Self {
        Self {
            base_trail_damage: TRAIL_BASE_DAMAGE as f32,
            bonus_multiplier: 0.0,
            accuracy_stacks: 0, // V2.5: Start with no accuracy
        }
    }
}

impl PlayerCombatStats {
    fn trail_damage(&self) -> f32 {
        self.base_trail_damage * (1.0 + self.bonus_multiplier)
    }

    fn add_bonus(&mut self) {
        self.bonus_multiplier += DAMAGE_POWER_BONUS;
    }

    fn reset(&mut self) {
        self.bonus_multiplier = 0.0;
    }
}

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
        self.multiplier = 1.0 + (self.streak.saturating_sub(1) as f32) * COMBO_MULTIPLIER_STEP;
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

#[derive(Component)]
struct HudScore;

#[derive(Component)]
struct HudHealth;

#[derive(Component)]
struct HudCombo;

#[derive(Component)]
struct HudBuffs;

#[derive(Component)]
struct HudStatus;

#[derive(Component)]
struct PowerUp {
    kind: PowerUpKind,
}

#[derive(Component)]
struct PowerUpLifetime(Timer);

// Sprite-based Particle System
#[derive(Component)]
struct Particle {
    velocity: Vec2,
    lifetime: Timer,
}

// V2: Wave Projectile (OLD - being replaced)
#[derive(Component)]
struct WaveProjectile {
    velocity: Vec2,
    lifetime: Timer,
    damage: u32,
}

// V2.5: Curved wave trail (fish-tail effect)
#[derive(Component)]
struct WaveTrail {
    spawn_time: f32,
    lifetime: f32,
    velocity: Vec2,
    curve_amount: f32,
    damage: u32,
}

// V2: Background Component
#[derive(Component)]
struct Background;

// V2.5: Background tile tracking
#[derive(Component)]
struct BackgroundTile {
    grid_x: i32,
    grid_y: i32,
}

#[derive(Resource)]
struct HitFreeze {
    timer: Timer,
}

impl Default for HitFreeze {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}

// V2.7: Font resource for emoji sprites
#[derive(Resource)]
struct GameFont(Handle<Font>);

#[derive(Clone, Copy)]
enum PowerUpKind {
    Heart,
    Shield,
    Damage,
    Accuracy, // V2.5: New power-up
}

impl Default for TrailSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            TRAIL_SPAWN_INTERVAL,
            TimerMode::Repeating,
        ))
    }
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera with screen shake
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
        ScreenShake::default(),
    ));

    // Load font for HUD and game entities
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.insert_resource(GameFont(font.clone()));
    
    // V2.7: Load background image (4K space image) - will be dynamically tiled
    let background_texture = asset_server.load("240_F_324745441_29s2iZ2NoUgq12WDBQcJ4CRjPn82Kc0D_imgupscaler.ai_General_4K.jpg");
    
    // V2.7: Spawn initial tiled background (large tiles for infinite feel)
    // Image is 3840x2158, scale it to fill large space
    let tile_width = 3840.0;
    let tile_height = 2158.0;
    
    for x in -2..=2 {
        for y in -2..=2 {
            commands.spawn((
                SpriteBundle {
                    texture: background_texture.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * tile_width,
                        y as f32 * tile_height,
                        -100.0, // Far back but visible
                    ),
                    sprite: Sprite {
                        color: Color::srgba(1.0, 1.0, 1.0, 0.4), // Dim for atmosphere
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Background,
                BackgroundTile {
                    grid_x: x,
                    grid_y: y,
                },
            ));
        }
    }

    // V2.7: Player as bright glowing sprite
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.2),
            sprite: Sprite {
                color: Color::srgba(0.4, 1.5, 2.0, 1.0), // SUPER bright cyan glow
                custom_size: Some(Vec2::splat(32.0)), // Larger player
                ..Default::default()
            },
            ..Default::default()
        },
        Player::default(),
        PlayerVelocity {
            current: Vec2::ZERO,
        },
        Knockback::default(),
    ));

    // HUD Container
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(16.0),
                left: Val::Px(16.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Score: 0 | Best: 0",
                    TextStyle {
                        font: font.clone(),
                        font_size: 22.0,
                        color: Color::WHITE,
                    },
                ),
                HudScore,
            ));

            parent.spawn((
                TextBundle::from_section(
                    format!("Health: {}/{}", PLAYER_MAX_HEALTH, PLAYER_MAX_HEALTH),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::srgb(1.0, 0.6, 0.7),
                    },
                ),
                HudHealth,
            ));

            parent.spawn((
                TextBundle::from_section(
                    "Combo x1.0 (0)",
                    TextStyle {
                        font: font.clone(),
                        font_size: 18.0,
                        color: Color::srgb(0.7, 0.9, 1.0),
                    },
                ),
                HudCombo,
            ));

            parent.spawn((
                TextBundle::from_section(
                    "Damage x1.0 | Shield: Ready",
                    TextStyle {
                        font: font.clone(),
                        font_size: 18.0,
                        color: Color::srgb(0.8, 0.8, 0.9),
                    },
                ),
                HudBuffs,
            ));

            parent.spawn((
                TextBundle::from_section(
                    "Status: Running",
                    TextStyle {
                        font: font.clone(),
                        font_size: 18.0,
                        color: Color::srgb(0.6, 0.6, 0.6),
                    },
                ),
                HudStatus,
            ));
        });
    
    // Initialize hit freeze
    commands.insert_resource(HitFreeze::default());
}

// Spawn death explosion particles (sprite-based)
fn spawn_death_explosion(commands: &mut Commands, position: Vec2) {
    use std::f32::consts::PI;
    let particle_count = 20;
    
    for i in 0..particle_count {
        let angle = (i as f32 / particle_count as f32) * PI * 2.0;
        let speed = 100.0 + rand::random::<f32>() * 100.0;
        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
        let size = 8.0 + rand::random::<f32>() * 4.0;
        
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(position.x, position.y, 0.4),
                sprite: Sprite {
                    color: Color::srgba(1.0, 0.5, 0.3, 1.0),
                    custom_size: Some(Vec2::splat(size)),
                    ..Default::default()
                },
                ..Default::default()
            },
            Particle {
                velocity,
                lifetime: Timer::from_seconds(0.6, TimerMode::Once),
            },
        ));
    }
}

// Spawn pickup ring particles
fn spawn_pickup_ring(commands: &mut Commands, position: Vec2, color: Color) {
    use std::f32::consts::PI;
    let particle_count = 12;
    let radius = 25.0;
    
    for i in 0..particle_count {
        let angle = (i as f32 / particle_count as f32) * PI * 2.0;
        let start_pos = position + Vec2::new(angle.cos(), angle.sin()) * radius;
        let velocity = Vec2::new(angle.cos(), angle.sin()) * 100.0;
        
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(start_pos.x, start_pos.y, 0.35),
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(6.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            Particle {
                velocity,
                lifetime: Timer::from_seconds(0.4, TimerMode::Once),
            },
        ));
    }
}

fn update_pointer_target(
    run_state: Res<RunState>,
    mut target: ResMut<PointerTarget>,
    mut motion_events: EventReader<MouseMotion>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };

    if let Ok((camera, transform)) = camera_query.get_single() {
        if let Some(cursor_position) = window.cursor_position() {
            if let Some(ray) = camera.viewport_to_world(transform, cursor_position) {
                target.position = ray.origin.truncate();
                clamp_vec2_to_bounds(&mut target.position);
            }
        } else if run_state.is_running() {
            let mut delta = Vec2::ZERO;
            for event in motion_events.read() {
                delta.x += event.delta.x;
                delta.y -= event.delta.y;
            }

            if delta.length_squared() > f32::EPSILON {
                target.position += delta;
                clamp_vec2_to_bounds(&mut target.position);
            }
        }
    }

    if !run_state.is_running() {
        motion_events.clear();
    }
}

fn move_player(
    time: Res<Time>,
    run_state: Res<RunState>,
    target: Res<PointerTarget>,
    stats: Res<PlayerCombatStats>,
    mut query: Query<(&mut Transform, &mut PlayerVelocity, &Knockback), With<Player>>,
) {
    if !run_state.is_running() {
        return;
    }

    let (mut transform, mut velocity, knockback) = query.single_mut();
    let current = transform.translation.truncate();
    let delta = target.position - current;
    let distance = delta.length();

    // V2.6 FIX: Accuracy as MULTIPLIER (safe), not addition (causes jitter)
    let accuracy_mult = 1.0 + (stats.accuracy_stacks as f32 * 0.12).min(0.4); // Max 1.4x
    
    // V2.6 FIX: Much snappier base values (was 0.12/0.25)
    let base_accel = 0.2;
    let base_decel = 0.4;
    
    // Apply accuracy as multiplier with HARD CAPS to prevent jitter
    let acceleration = (base_accel * accuracy_mult).min(0.45);
    let deceleration = (base_decel * accuracy_mult).min(0.65);
    let max_speed = PLAYER_SPEED * accuracy_mult;

    // Smooth acceleration/deceleration with momentum
    if distance > 5.0 {
        let desired = delta.normalize() * max_speed;
        velocity.current = velocity.current.lerp(desired, acceleration);
    } else {
        // Decelerate when near target
        velocity.current = velocity.current.lerp(Vec2::ZERO, deceleration);
    }

    // Apply velocity and knockback
    let combined_velocity = velocity.current + knockback.velocity;
    transform.translation += combined_velocity.extend(0.0) * time.delta_seconds();
    
    clamp_to_bounds(&mut transform.translation);
}

fn tick_shield_state(time: Res<Time>, mut shield: ResMut<ShieldState>) {
    if let Some(timer) = shield.timer.as_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            shield.clear();
        }
    }
}

fn clamp_to_bounds(translation: &mut Vec3) {
    // V2.5 FIX: Use ARENA_SIZE for large playable area, not ARENA_BOUNDS!
    let half_size = ARENA_SIZE * 0.5; // 2500 units radius
    translation.x = translation.x.clamp(-half_size, half_size);
    translation.y = translation.y.clamp(-half_size, half_size);
}

fn clamp_vec2_to_bounds(position: &mut Vec2) {
    // V2.5 FIX: Use ARENA_SIZE for large playable area!
    let half_size = ARENA_SIZE * 0.5; // 2500 units radius
    position.x = position.x.clamp(-half_size, half_size);
    position.y = position.y.clamp(-half_size, half_size);
}

fn spawn_trail_segments(
    mut commands: Commands,
    time: Res<Time>,
    run_state: Res<RunState>,
    mut timer: ResMut<TrailSpawnTimer>,
    player_query: Query<(&Transform, &Player)>,
) {
    if !run_state.is_running() {
        return;
    }
    
    // V2: Only spawn trail in Trail mode
    let Ok((transform, player)) = player_query.get_single() else {
        return;
    };
    
    if player.weapon_type != WeaponType::Trail {
        return;
    }

    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }

    timer.0.reset();

    let position = transform.translation;

    // V2: Brighter trail with glow
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(position.x, position.y, 0.1),
            sprite: Sprite {
                color: Color::srgba(0.5, 1.2, 1.4, 0.95), // Much brighter cyan
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

// V2.7: Spawn enemies as emoji
fn spawn_enemies(
    mut commands: Commands,
    game_font: Res<GameFont>,
    time: Res<Time>,
    run_state: Res<RunState>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    score: Res<Score>,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    if !run_state.is_running() {
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

    // V2: Spawn relative to camera position (infinite space)
    let camera_pos = if let Ok(camera_transform) = camera_query.get_single() {
        camera_transform.translation.truncate()
    } else {
        Vec2::ZERO
    };

    let mut rng = thread_rng();
    let angle = rng.gen::<f32>() * std::f32::consts::TAU; // Random angle around camera
    let distance = ENEMY_SPAWN_DISTANCE; // Spawn just off-screen
    let position = camera_pos + Vec2::from_angle(angle) * distance;

    let speed_bonus = ENEMY_SPEED_INCREMENT * score.current as f32 / 200.0;
    let enemy_speed = ENEMY_BASE_SPEED + speed_bonus;

    // V2.7: Enemy as bright glowing sprite
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(position.x, position.y, 0.15),
            sprite: Sprite {
                color: Color::srgba(2.0, 0.3, 0.4, 1.0), // SUPER bright red glow
                custom_size: Some(Vec2::new(40.0, 36.0)), // Rectangular enemies
                ..Default::default()
            },
            ..Default::default()
        },
        Enemy { speed: enemy_speed },
        EnemyHealth {
            current: ENEMY_BASE_HEALTH as f32,
        },
        EnemyVelocity {
            current: Vec2::ZERO,
        },
        Knockback::default(),
    ));
}

fn move_enemies(
    time: Res<Time>,
    run_state: Res<RunState>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies: Query<(&Enemy, &mut Transform, &mut EnemyVelocity, &Knockback), Without<Player>>,
) {
    if !run_state.is_running() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    
    for (enemy, mut transform, mut velocity, knockback) in &mut enemies {
        let current_pos = transform.translation.truncate();
        let direction = player_pos - current_pos;
        
        if direction.length() > f32::EPSILON {
            // Steering behavior - smooth turning
            let desired = direction.normalize() * enemy.speed;
            let steering = desired - velocity.current;
            let steering_force = steering.clamp_length_max(enemy.speed * ENEMY_TURN_SPEED);
            
            velocity.current += steering_force * time.delta_seconds() * 10.0;
            velocity.current = velocity.current.clamp_length_max(enemy.speed);
            
            // Apply velocity and knockback
            let combined = velocity.current + knockback.velocity;
            transform.translation += combined.extend(0.0) * time.delta_seconds();
        }
    }
}

// V2.7: Added game_font for power-up spawning
fn handle_trail_collisions(
    mut commands: Commands,
    game_font: Res<GameFont>,
    run_state: Res<RunState>,
    mut score: ResMut<Score>,
    mut combo: ResMut<Combo>,
    mut rng: Local<Option<StdRng>>,
    combat: Res<PlayerCombatStats>,
    mut hit_freeze: ResMut<HitFreeze>,
    mut camera_query: Query<&mut ScreenShake, With<MainCamera>>,
    mut enemies: Query<(Entity, &Transform, &Enemy, &mut EnemyHealth, &mut Knockback)>,
    trail: Query<&Transform, With<TrailSegment>>,
) {
    if !run_state.is_running() {
        return;
    }

    let damage = combat.trail_damage();
    let rng = rng.get_or_insert_with(|| StdRng::from_rng(thread_rng()).unwrap());
    let mut defeated = Vec::new();
    let mut camera_shake = camera_query.single_mut();

    for (enemy_entity, enemy_transform, _enemy, mut health, mut knockback) in &mut enemies {
        let enemy_pos = enemy_transform.translation.truncate();
        let mut hit = false;
        let mut hit_direction = Vec2::ZERO;
        
        for segment_transform in &trail {
            let diff = enemy_pos - segment_transform.translation.truncate();
            if diff.length_squared() <= TRAIL_HIT_RADIUS * TRAIL_HIT_RADIUS {
                hit = true;
                hit_direction = diff.normalize_or_zero();
                break;
            }
        }

        if hit {
            health.current -= damage;
            
            // Apply knockback
            knockback.velocity = hit_direction * ENEMY_KNOCKBACK;
            
            if health.current <= 0.0 {
                defeated.push((enemy_entity, enemy_pos));
            }
        }
    }

    if !defeated.is_empty() {
        for (entity, position) in defeated {
            score.current += combo.register_kill();
            
            // JUICE: Screen shake based on combo
            let shake_amount = 0.2 + (combo.streak as f32 * 0.05).min(0.4);
            camera_shake.trauma = (camera_shake.trauma + shake_amount).min(1.0);
            
            // JUICE: Hit freeze
            hit_freeze.timer = Timer::from_seconds(HIT_FREEZE_DURATION, TimerMode::Once);
            
            // JUICE: Spawn death explosion particles
            spawn_death_explosion(&mut commands, position);
            
            maybe_spawn_power_up(&mut commands, &game_font.0, rng, position);
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn handle_player_collisions(
    mut commands: Commands,
    mut run_state: ResMut<RunState>,
    mut score: ResMut<Score>,
    mut combo: ResMut<Combo>,
    mut health: ResMut<PlayerHealth>,
    shield: Res<ShieldState>,
    mut camera_query: Query<&mut ScreenShake, With<MainCamera>>,
    mut player_query: Query<(&Transform, &mut Knockback), With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    if !run_state.is_running() {
        return;
    }

    let (player_transform, mut player_knockback) = player_query.single_mut();
    let player_pos = player_transform.translation.truncate();
    let mut camera_shake = camera_query.single_mut();

    for (entity, transform) in &enemies {
        let enemy_pos = transform.translation.truncate();
        let diff = enemy_pos - player_pos;
        
        if diff.length_squared() <= PLAYER_RADIUS * PLAYER_RADIUS {
            // Despawn enemy regardless of shield
            commands.entity(entity).despawn_recursive();
            
            // Only apply damage if shield is not active
            if !shield.is_active() {
                health.apply_damage(PLAYER_COLLISION_DAMAGE);
                
                // JUICE: Heavy screen shake when player gets hit
                camera_shake.trauma = (camera_shake.trauma + 0.5).min(1.0);
                
                // JUICE: Knockback player away from enemy
                let knockback_dir = (player_pos - enemy_pos).normalize_or_zero();
                player_knockback.velocity = knockback_dir * PLAYER_KNOCKBACK_STRENGTH;
                
                if health.current == 0 {
                    run_state.active = false;
                    score.best = score.best.max(score.current);
                    combo.reset();
                    
                    // JUICE: Massive screen shake on death
                    camera_shake.trauma = 1.0;
                }
            }
            break;
        }
    }
}

fn handle_power_up_pickups(
    mut commands: Commands,
    run_state: Res<RunState>,
    mut player_health: ResMut<PlayerHealth>,
    mut shield: ResMut<ShieldState>,
    mut combat: ResMut<PlayerCombatStats>,
    mut camera_query: Query<&mut ScreenShake, With<MainCamera>>,
    player_query: Query<&Transform, With<Player>>,
    mut power_ups: Query<(Entity, &Transform, &PowerUp)>,
) {
    if !run_state.is_running() {
        return;
    }

    let player_transform = player_query.single();
    let player_pos = player_transform.translation.truncate();
    let mut camera_shake = camera_query.single_mut();

    for (entity, transform, power_up) in &mut power_ups {
        let diff = transform.translation.truncate() - player_pos;
        if diff.length_squared() <= PLAYER_RADIUS * PLAYER_RADIUS {
            let particle_color = match power_up.kind {
                PowerUpKind::Heart => {
                    player_health.heal(1);
                    Color::srgba(1.0, 0.5, 0.6, 1.0)
                }
                PowerUpKind::Shield => {
                    shield.activate();
                    Color::srgba(0.5, 0.8, 1.0, 1.0)
                }
                PowerUpKind::Damage => {
                    combat.add_bonus();
                    Color::srgba(1.0, 0.8, 0.3, 1.0)
                }
                PowerUpKind::Accuracy => {
                    // V2.5: Grant accuracy - makes movement snappier
                    combat.accuracy_stacks += 1;
                    Color::srgba(1.0, 0.4, 1.0, 1.0) // Purple
                }
            };

            // JUICE: Light screen shake on pickup
            camera_shake.trauma = (camera_shake.trauma + 0.15).min(1.0);
            
            // JUICE: Spawn pickup ring particles
            spawn_pickup_ring(&mut commands, transform.translation.truncate(), particle_color);

            commands.entity(entity).despawn_recursive();
        }
    }
}

fn tick_power_up_lifetimes(
    time: Res<Time>,
    mut commands: Commands,
    mut power_ups: Query<(Entity, &mut PowerUpLifetime)>,
) {
    for (entity, mut lifetime) in &mut power_ups {
        if lifetime.0.tick(time.delta()).finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn tick_combo(time: Res<Time>, mut combo: ResMut<Combo>, run_state: Res<RunState>) {
    if !run_state.is_running() {
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
    shield: Res<ShieldState>,
    combat: Res<PlayerCombatStats>,
    mut score_text: Query<&mut Text, With<HudScore>>,
    mut health_text: Query<
        &mut Text,
        (
            With<HudHealth>,
            Without<HudScore>,
            Without<HudCombo>,
            Without<HudBuffs>,
            Without<HudStatus>,
        ),
    >,
    mut combo_text: Query<
        &mut Text,
        (
            With<HudCombo>,
            Without<HudScore>,
            Without<HudHealth>,
            Without<HudBuffs>,
            Without<HudStatus>,
        ),
    >,
    mut buffs_text: Query<
        &mut Text,
        (
            With<HudBuffs>,
            Without<HudScore>,
            Without<HudHealth>,
            Without<HudCombo>,
            Without<HudStatus>,
        ),
    >,
    mut status_text: Query<
        &mut Text,
        (
            With<HudStatus>,
            Without<HudScore>,
            Without<HudHealth>,
            Without<HudCombo>,
            Without<HudBuffs>,
        ),
    >,
) {
    // Update score
    if let Ok(mut text) = score_text.get_single_mut() {
        text.sections[0].value = format!("Score: {} | Best: {}", score.current, score.best);
    }

    // Update health
    if let Ok(mut text) = health_text.get_single_mut() {
        text.sections[0].value = format!("Health: {}/{}", health.current, health.max);
    }

    // Update combo
    if let Ok(mut text) = combo_text.get_single_mut() {
        text.sections[0].value = format!("Combo x{:.1} ({})", combo.multiplier, combo.streak);
    }

    // Update buffs
    if let Ok(mut text) = buffs_text.get_single_mut() {
        let damage_multiplier = combat.trail_damage() / combat.base_trail_damage;
        let shield_text = if let Some(remaining) = shield.remaining_seconds() {
            format!("Shield: {:.1}s", remaining)
        } else {
            "Shield: Ready".to_string()
        };
        text.sections[0].value = format!("Damage x{:.1} | {}", damage_multiplier, shield_text);
    }

    // Update status
    if let Ok(mut text) = status_text.get_single_mut() {
        let status = if !run_state.active {
            "Status: Down! Press SPACE to respawn."
        } else if run_state.paused {
            "Status: Paused - Press ESC to resume."
        } else {
            "Status: Running"
        };
        text.sections[0].value = status.to_string();
    }
}

fn handle_restart(
    keys: Res<ButtonInput<KeyCode>>,
    mut run_state: ResMut<RunState>,
    mut score: ResMut<Score>,
    mut combo: ResMut<Combo>,
    mut pointer: ResMut<PointerTarget>,
    mut health: ResMut<PlayerHealth>,
    mut shield: ResMut<ShieldState>,
    mut combat: ResMut<PlayerCombatStats>,
    mut enemy_spawn: ResMut<EnemySpawnTimer>,
    mut trail_timer: ResMut<TrailSpawnTimer>,
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
    run_state.paused = false;
    score.current = 0;
    combo.reset();
    pointer.reset();
    health.reset();
    shield.clear();
    combat.reset();
    enemy_spawn.timer = Timer::from_seconds(ENEMY_SPAWN_INTERVAL_START, TimerMode::Repeating);
    trail_timer.0 = Timer::from_seconds(TRAIL_SPAWN_INTERVAL, TimerMode::Repeating);

    for entity in enemies.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in trail_segments.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let mut player_transform = player_query.single_mut();
    player_transform.translation = Vec3::new(0.0, 0.0, player_transform.translation.z);
    pointer.position = player_transform.translation.truncate();
}

fn enforce_cursor_lock(
    run_state: Res<RunState>,
    mut state: ResMut<CursorLockState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };

    // On web, cursor lock requires user interaction
    // Lock cursor on first click when game is running
    if run_state.is_running() && !state.locked {
        if mouse_button.just_pressed(MouseButton::Left) || mouse_button.just_pressed(MouseButton::Right) {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
            state.locked = true;
        }
    } else if !run_state.is_running() && state.locked {
        // Unlock when game stops or pauses
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
        state.locked = false;
    }
}

fn handle_pause_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    mut run_state: ResMut<RunState>,
    mut cursor_state: ResMut<CursorLockState>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if !run_state.active {
        return;
    }

    if keys.just_pressed(KeyCode::Escape) {
        run_state.paused = !run_state.paused;
        
        // Force cursor unlock when pausing
        if run_state.paused {
            if let Ok(mut window) = windows.get_single_mut() {
                window.cursor.visible = true;
                window.cursor.grab_mode = CursorGrabMode::None;
                cursor_state.locked = false;
            }
        }
    }
}

// V2.7: Updated to pass game font for emoji
fn maybe_spawn_power_up(commands: &mut Commands, game_font: &Handle<Font>, rng: &mut StdRng, position: Vec2) {
    if rng.gen::<f32>() > POWER_UP_DROP_CHANCE {
        return;
    }

    // V2.5: Updated for 4 power-up types
    let total_weight = POWER_UP_HEART_WEIGHT + POWER_UP_SHIELD_WEIGHT + POWER_UP_DAMAGE_WEIGHT + POWER_UP_ACCURACY_WEIGHT;
    let mut roll = rng.gen::<f32>() * total_weight;

    let kind = if roll < POWER_UP_HEART_WEIGHT {
        PowerUpKind::Heart
    } else {
        roll -= POWER_UP_HEART_WEIGHT;
        if roll < POWER_UP_SHIELD_WEIGHT {
            PowerUpKind::Shield
        } else {
            roll -= POWER_UP_SHIELD_WEIGHT;
            if roll < POWER_UP_DAMAGE_WEIGHT {
                PowerUpKind::Damage
            } else {
                PowerUpKind::Accuracy // V2.5: New option
            }
        }
    };

    spawn_power_up(commands, game_font, position, kind);
}

// V2.7: Spawn power-ups with distinct shapes and colors
fn spawn_power_up(commands: &mut Commands, _game_font: &Handle<Font>, position: Vec2, kind: PowerUpKind) {
    // Distinct size and color for each power-up type
    let (color, size) = match kind {
        PowerUpKind::Heart => (
            Color::srgba(2.2, 0.2, 0.3, 1.0),  // SUPER bright red
            Vec2::splat(26.0)                   // Circle (heart)
        ),
        PowerUpKind::Shield => (
            Color::srgba(0.3, 1.8, 2.2, 1.0),  // SUPER bright cyan
            Vec2::new(28.0, 32.0)               // Tall rectangle (shield)
        ),
        PowerUpKind::Damage => (
            Color::srgba(2.5, 1.5, 0.2, 1.0),  // SUPER bright gold
            Vec2::new(22.0, 22.0)               // Diamond shape (sword)
        ),
        PowerUpKind::Accuracy => (
            Color::srgba(2.0, 0.2, 2.2, 1.0),  // SUPER bright purple
            Vec2::splat(24.0)                   // Circle (target)
        ),
    };

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(position.x, position.y, 0.12),
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..Default::default()
            },
            ..Default::default()
        },
        PowerUp { kind },
        PowerUpLifetime(Timer::from_seconds(POWER_UP_LIFETIME, TimerMode::Once)),
    ));
}

// ========== V2: NEW SYSTEMS ==========

// V2: Camera follows player smoothly
fn camera_follow_player(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let target = player_transform.translation.truncate();
            
            // Smooth lerp follow
            camera_transform.translation.x += (target.x - camera_transform.translation.x) * CAMERA_SMOOTHING;
            camera_transform.translation.y += (target.y - camera_transform.translation.y) * CAMERA_SMOOTHING;
            camera_transform.translation.z = 999.9; // Keep camera Z fixed
        }
    }
}

// V2.5: Infinite parallax background - dynamically spawn/despawn tiles
// V2.7: Dynamic background tiling with correct dimensions and parallax depth
fn update_background_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<MainCamera>>,
    bg_tiles: Query<(Entity, &BackgroundTile)>,
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };
    
    let camera_pos = camera_transform.translation.truncate();
    
    // V2.7: Image is 3840x2158
    let tile_width = 3840.0;
    let tile_height = 2158.0;
    let tile_size = Vec2::new(tile_width, tile_height);
    
    // Calculate which tile grid we're in
    let grid_x = (camera_pos.x / tile_size.x).floor() as i32;
    let grid_y = (camera_pos.y / tile_size.y).floor() as i32;
    
    // Keep a 5x5 grid around camera (infinite tiling)
    let mut needed_tiles = std::collections::HashSet::new();
    for dx in -2..=2 {
        for dy in -2..=2 {
            needed_tiles.insert((grid_x + dx, grid_y + dy));
        }
    }
    
    // Remove tiles too far from camera
    for (entity, tile) in &bg_tiles {
        if !needed_tiles.contains(&(tile.grid_x, tile.grid_y)) {
            commands.entity(entity).despawn_recursive();
        }
    }
    
    // Find existing tiles
    let existing_tiles: std::collections::HashSet<(i32, i32)> = 
        bg_tiles.iter().map(|(_, tile)| (tile.grid_x, tile.grid_y)).collect();
    
    // Spawn missing tiles
    let background_texture = asset_server.load(
        "240_F_324745441_29s2iZ2NoUgq12WDBQcJ4CRjPn82Kc0D_imgupscaler.ai_General_4K.jpg"
    );
    
    for (gx, gy) in needed_tiles {
        if !existing_tiles.contains(&(gx, gy)) {
            commands.spawn((
                SpriteBundle {
                    texture: background_texture.clone(),
                    transform: Transform::from_xyz(
                        gx as f32 * tile_width,
                        gy as f32 * tile_height,
                        -100.0, // Far back but visible
                    ),
                    sprite: Sprite {
                        color: Color::srgba(1.0, 1.0, 1.0, 0.4), // Dim for atmosphere
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Background,
                BackgroundTile { grid_x: gx, grid_y: gy },
            ));
        }
    }
}

// V2: Toggle weapon mode
fn toggle_weapon(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
) {
    if keys.just_pressed(KeyCode::KeyQ) {
        if let Ok(mut player) = player_query.get_single_mut() {
            player.weapon_type = match player.weapon_type {
                WeaponType::Trail => WeaponType::Wave,
                WeaponType::Wave => WeaponType::Trail,
            };
        }
    }
}

// V2.6: Powerful ocean wave casting (COMPLETE REDESIGN)
fn spawn_wave_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    run_state: Res<RunState>,
    player_query: Query<(&Transform, &PlayerVelocity, &Player)>,
    mut wave_timer: Local<f32>,
) {
    if !run_state.is_running() {
        return;
    }
    
    let Ok((player_transform, velocity, player)) = player_query.get_single() else {
        return;
    };
    
    if player.weapon_type != WeaponType::Wave {
        return;
    }
    
    // V2.6 FIX: Lower threshold (was 50.0 - way too high!)
    if velocity.current.length_squared() < 5000.0 {
        return;
    }
    
    // V2.6 FIX: Consistent timing (no modulo gaps)
    *wave_timer += time.delta_seconds();
    if *wave_timer < 0.08 {
        return;
    }
    *wave_timer = 0.0;
    
    let player_pos = player_transform.translation.truncate();
    let move_dir = velocity.current.normalize_or_zero();
    
    if move_dir == Vec2::ZERO {
        return;
    }
    
    // V2.6: 6 waves total (3 per side) for POWERFUL visual impact
    for side in [-1.0, 1.0] {
        let perpendicular = Vec2::new(-move_dir.y, move_dir.x) * side;
        
        for i in 0..3 {
            // V2.6: Spawn FAR from player (creates "casting" effect, not "falling off")
            let distance_out = 35.0 + (i as f32 * 18.0); // 35, 53, 71px out
            let distance_back = -(i as f32 * 8.0); // Slightly behind
            let spawn_offset = perpendicular * distance_out + move_dir * distance_back;
            
            // V2.6: STRONG outward velocity (creates sweeping arc)
            let strength = 1.3 - (i as f32 * 0.1); // Inner waves faster
            let curve_velocity = (perpendicular * strength + move_dir * 0.2) * 500.0;
            
            // V2.6: MUCH LARGER particles (was 16px - looked like droplets!)
            let wave_size = 28.0 + (i as f32 * 10.0); // 28, 38, 48px
            
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        player_pos.x + spawn_offset.x,
                        player_pos.y + spawn_offset.y,
                        0.15,
                    ),
                    sprite: Sprite {
                        color: Color::srgba(0.2, 0.9, 1.6, 1.0), // VERY bright cyan
                        custom_size: Some(Vec2::splat(wave_size)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                WaveTrail {
                    spawn_time: time.elapsed_seconds(),
                    lifetime: 2.0, // Longer (was 1.2s)
                    velocity: curve_velocity,
                    curve_amount: side * 400.0, // MUCH stronger curves (was 180)
                    damage: 2,
                },
            ));
        }
    }
}

// V2.6: Update powerful ocean waves with strong curves
fn update_wave_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut trails: Query<(Entity, &mut Transform, &mut Sprite, &mut WaveTrail)>,
) {
    let current_time = time.elapsed_seconds();
    
    for (entity, mut transform, mut sprite, mut wave) in &mut trails {
        let age = current_time - wave.spawn_time;
        
        if age > wave.lifetime {
            commands.entity(entity).despawn_recursive();
            continue;
        }
        
        // V2.6: STRONGER curved motion (powerful sweeping arcs)
        let curve_perpendicular = Vec2::new(-wave.velocity.y, wave.velocity.x).normalize_or_zero();
        let curve_force = curve_perpendicular * wave.curve_amount * time.delta_seconds();
        wave.velocity += curve_force;
        
        // V2.6: LESS friction (travels much farther, was 0.985)
        wave.velocity *= 0.992;
        
        // Move the wave
        transform.translation += wave.velocity.extend(0.0) * time.delta_seconds();
        
        // V2.6: SLOWER fade (stays visible longer)
        let life_percent = 1.0 - (age / wave.lifetime);
        sprite.color.set_alpha((life_percent * 1.2).min(1.0));
        
        // V2.6: GROWS as it expands (ocean wave effect, was shrinking!)
        let scale = 1.0 + (1.0 - life_percent) * 0.3;
        transform.scale = Vec3::splat(scale);
    }
}

// V2.5: Wave trail collision with enemies
// V2.7: Added game_font for power-up spawning
fn handle_wave_collisions(
    mut commands: Commands,
    game_font: Res<GameFont>,
    mut score: ResMut<Score>,
    mut combo: ResMut<Combo>,
    mut rng: Local<Option<StdRng>>,
    mut hit_freeze: ResMut<HitFreeze>,
    mut camera_query: Query<&mut ScreenShake, With<MainCamera>>,
    trails: Query<&Transform, With<WaveTrail>>,
    mut enemies: Query<(Entity, &Transform, &mut EnemyHealth, &mut Knockback), With<Enemy>>,
) {
    let rng = rng.get_or_insert_with(|| StdRng::from_rng(thread_rng()).unwrap());
    let mut camera_shake = camera_query.single_mut();
    
    for trail_transform in &trails {
        let trail_pos = trail_transform.translation.truncate();
        
        for (enemy_entity, enemy_transform, mut health, mut knockback) in &mut enemies {
            let enemy_pos = enemy_transform.translation.truncate();
            let distance = trail_pos.distance(enemy_pos);
            
            if distance < 24.0 {
                health.current -= 2.0; // Wave damage
                
                let knock_dir = (enemy_pos - trail_pos).normalize_or_zero();
                knockback.velocity = knock_dir * 250.0;
                
                if health.current <= 0.0 {
                    score.current += combo.register_kill();
                    camera_shake.trauma = (camera_shake.trauma + 0.15).min(1.0);
                    hit_freeze.timer = Timer::from_seconds(HIT_FREEZE_DURATION, TimerMode::Once);
                    spawn_death_explosion(&mut commands, enemy_pos);
                    maybe_spawn_power_up(&mut commands, &game_font.0, rng, enemy_pos);
                    commands.entity(enemy_entity).despawn_recursive();
                }
                
                break;
            }
        }
    }
}

// ========== GAME FEEL SYSTEMS ==========

// Screen shake system - makes camera shake based on trauma
fn update_screen_shake(
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut ScreenShake), With<MainCamera>>,
) {
    for (mut transform, mut shake) in &mut camera_query {
        if shake.trauma > 0.0 {
            shake.trauma = (shake.trauma - SCREEN_SHAKE_DECAY * time.delta_seconds()).max(0.0);
            
            let trauma_sq = shake.trauma * shake.trauma;
            let offset = Vec3::new(
                (rand::random::<f32>() - 0.5) * trauma_sq * 20.0,
                (rand::random::<f32>() - 0.5) * trauma_sq * 20.0,
                0.0,
            );
            
            transform.translation.x = offset.x;
            transform.translation.y = offset.y;
        } else if transform.translation.x != 0.0 || transform.translation.y != 0.0 {
            // Reset camera position when shake is done
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
        }
    }
}

// Knockback system - applies and decays knockback velocity
fn apply_knockback(
    time: Res<Time>,
    mut query: Query<&mut Knockback>,
) {
    for mut knockback in &mut query {
        if knockback.velocity.length_squared() > 1.0 {
            knockback.velocity *= 1.0 - (8.0 * time.delta_seconds());
        } else {
            knockback.velocity = Vec2::ZERO;
        }
    }
}

// Hit freeze system - brief pause for impact feel
fn tick_hit_freeze(
    time: Res<Time>,
    mut hit_freeze: ResMut<HitFreeze>,
) {
    if !hit_freeze.timer.finished() {
        hit_freeze.timer.tick(time.delta());
    }
}

// Update and cleanup sprite particles
fn despawn_finished_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Sprite, &mut Particle)>,
) {
    for (entity, mut transform, mut sprite, mut particle) in &mut query {
        particle.lifetime.tick(time.delta());
        
        // Move particle
        transform.translation += particle.velocity.extend(0.0) * time.delta_seconds();
        
        // Fade out based on remaining lifetime
        let life_percent = particle.lifetime.fraction_remaining();
        sprite.color.set_alpha(life_percent);
        
        // Despawn when lifetime expires
        if particle.lifetime.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
