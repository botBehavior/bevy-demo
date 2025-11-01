use bevy::input::gamepad::{GamepadAxisType, GamepadEvent};
use bevy::input::touch::{TouchInput, TouchPhase};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

#[cfg(target_arch = "wasm32")]
use getrandom as _;
use std::f32::consts::TAU;
use std::time::Duration;
use threadweaver_core::components::*;
use threadweaver_core::constants::*;
use threadweaver_core::resources::*;
use threadweaver_core::shop::{UpgradeType, SHOP_ITEMS};
use threadweaver_core::util::{clamp_to_bounds, screen_to_world};
use threadweaver_platform::{load_currency, load_upgrades, save_currency, save_upgrades};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RunState::new())
            .insert_resource(PointerTarget::default())
            .insert_resource(PlayerStats::default())
            .insert_resource(PlayerHealth::with_max(PLAYER_MAX_HEALTH))
            .insert_resource(ShieldState {
                remaining: 0.0,
                duration: SHIELD_DURATION,
            })
            .insert_resource(Score::default())
            .insert_resource(Currency {
                balance: load_currency(),
            })
            .insert_resource(PurchasedUpgrades::default())
            .insert_resource(ShopState::default())
            .insert_resource(EnemySpawnTimer::new(ENEMY_SPAWN_INTERVAL_START))
            .insert_resource(TrailSpawnTimer {
                timer: Timer::from_seconds(TRAIL_SPAWN_INTERVAL, TimerMode::Repeating),
            })
            .add_event::<PlayerHitEvent>()
            .add_event::<ShopPurchaseEvent>()
            .add_systems(Startup, setup_scene)
            .add_systems(PostStartup, prime_persistence)
            .add_systems(
                Update,
                (
                    (
                        read_pointer_input,
                        read_touch_input,
                        read_gamepad_input,
                        move_player,
                        spawn_trail_segments,
                        update_trail_segments,
                        spawn_enemies,
                        move_enemies,
                        resolve_trail_hits,
                        resolve_player_collisions,
                    )
                        .chain(),
                    (
                        tick_powerups,
                        apply_powerup_pickups,
                        update_shield_state,
                        advance_wave_projectile_timer,
                        update_wave_projectiles,
                        update_particles,
                        apply_screen_shake,
                    )
                        .chain(),
                    handle_player_hit_events,
                    (
                        apply_shop_purchases,
                        persist_currency_changes,
                        persist_upgrade_changes,
                    )
                        .chain(),
                )
                    .chain(),
            )
            .add_systems(Update, reset_when_run_stops.after(handle_player_hit_events));
    }
}

fn prime_persistence(
    mut commands: Commands,
    mut health: ResMut<PlayerHealth>,
    mut shield: ResMut<ShieldState>,
) {
    let upgrades = load_upgrades();
    let mut upgraded_health = PLAYER_MAX_HEALTH + upgrades.max_health_bonus();
    if upgraded_health == 0 {
        upgraded_health = PLAYER_MAX_HEALTH;
    }
    health.max = upgraded_health;
    health.current = upgraded_health;
    shield.duration = SHIELD_DURATION + upgrades.shield_duration_bonus();
    shield.remaining = 0.0;

    commands.insert_resource(upgrades);
}

#[derive(Event)]
struct PlayerHitEvent;

#[derive(Event)]
pub struct ShopPurchaseEvent {
    pub item: UpgradeType,
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background = asset_server.load("background_tile.png");
    let player_texture = asset_server.load("sprites/player.png");
    let enemy_texture = asset_server.load("sprites/enemy_basic.png");
    let trail_texture = asset_server.load("sprites/effects/trail_segment.png");
    let wave_texture = asset_server.load("sprites/effects/wave_projectile.png");
    let currency_texture = asset_server.load("sprites/powerups/currency.png");
    let heart_texture = asset_server.load("sprites/powerups/heart.png");
    let shield_texture = asset_server.load("sprites/powerups/shield.png");
    let accuracy_texture = asset_server.load("sprites/powerups/accuracy.png");
    let waveblast_texture = asset_server.load("sprites/powerups/wave_blast.png");
    let font_primary = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.insert_resource(GameAssets {
        font_primary: font_primary.clone(),
        font_numbers: font_primary.clone(),
        player: player_texture.clone(),
        enemy: enemy_texture,
        background: background.clone(),
        trail_segment: trail_texture,
        wave_projectile: wave_texture,
        powerup_currency: currency_texture,
        powerup_health: heart_texture,
        powerup_shield: shield_texture,
        powerup_accuracy: accuracy_texture,
        powerup_waveblast: waveblast_texture,
    });

    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
        ScreenShake::default(),
    ));

    // Spawn tiled background instead of single large sprite
    spawn_background_tiles(&mut commands, background);

    commands.spawn((
        SpriteBundle {
            texture: player_texture,
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(48.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Player::default(),
        PlayerVelocity::default(),
        Knockback::default(),
    ));
}

fn read_pointer_input(
    mut events: EventReader<CursorMoved>,
    mut target: ResMut<PointerTarget>,
    run_state: Res<RunState>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if !run_state.is_running() {
        return;
    }

    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok((camera, transform)) = camera_q.get_single() else {
        return;
    };

    for event in events.read() {
        if let Some(position) = screen_to_world(camera, transform, event.position) {
            target.position = position;
            clamp_to_bounds(&mut target.position, ARENA_BOUNDS);
        }
    }

    if window.cursor_position().is_none() {
        // keep pointer inside arena when cursor hidden
        clamp_to_bounds(&mut target.position, ARENA_BOUNDS);
    }
}

fn read_touch_input(
    mut touch_events: EventReader<TouchInput>,
    mut target: ResMut<PointerTarget>,
    run_state: Res<RunState>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if !run_state.is_running() {
        return;
    }

    if windows.get_single().is_err() {
        return;
    }
    let Ok((camera, transform)) = camera_q.get_single() else {
        return;
    };

    for touch in touch_events.read() {
        if touch.phase == TouchPhase::Ended {
            continue;
        }

        if let Some(position) = screen_to_world(camera, transform, touch.position) {
            if target.position.distance_squared(position)
                > TOUCH_DRAG_DEADZONE * TOUCH_DRAG_DEADZONE
            {
                target.position = position;
                clamp_to_bounds(&mut target.position, ARENA_BOUNDS);
            }
        }
    }

    clamp_to_bounds(&mut target.position, ARENA_BOUNDS);
}

fn read_gamepad_input(
    mut events: EventReader<GamepadEvent>,
    mut target: ResMut<PointerTarget>,
    run_state: Res<RunState>,
) {
    if !run_state.is_running() {
        return;
    }

    for event in events.read() {
        if let GamepadEvent::Axis(axis_event) = event {
            match axis_event.axis_type {
                GamepadAxisType::LeftStickX => target.position.x += axis_event.value * 12.0,
                GamepadAxisType::LeftStickY => target.position.y += axis_event.value * 12.0,
                _ => {}
            }
        }
    }

    clamp_to_bounds(&mut target.position, ARENA_BOUNDS);
}

fn move_player(
    time: Res<Time>,
    stats: Res<PlayerStats>,
    upgrades: Res<PurchasedUpgrades>,
    target: Res<PointerTarget>,
    run_state: Res<RunState>,
    mut query: Query<
        (
            &mut Transform,
            &mut PlayerVelocity,
            &mut Player,
            &mut Knockback,
        ),
        With<Player>,
    >,
) {
    if !run_state.is_running() {
        return;
    }

    let Ok((mut transform, mut velocity, mut player, mut knockback)) = query.get_single_mut()
    else {
        return;
    };
    let mut current = transform.translation.truncate();
    let delta = target.position - current;
    let distance = delta.length();

    let speed = stats.base_speed * upgrades.movement_speed_multiplier();
    let accel = stats.acceleration;
    let decel = stats.deceleration;

    if distance > 4.0 {
        let desired = delta.normalize() * speed;
        velocity.current = velocity.current.lerp(desired, accel);
    } else {
        velocity.current = velocity.current.lerp(Vec2::ZERO, decel);
    }

    velocity.current += knockback.velocity;
    knockback.velocity *= 0.90; // damp knockback

    current += velocity.current * time.delta_seconds();
    clamp_to_bounds(&mut current, ARENA_BOUNDS);
    transform.translation.x = current.x;
    transform.translation.y = current.y;

    player.wave_cooldown = (player.wave_cooldown - time.delta_seconds()).max(0.0);
}

fn spawn_trail_segments(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<GameAssets>,
    run_state: Res<RunState>,
    mut timer: ResMut<TrailSpawnTimer>,
    upgrades: Res<PurchasedUpgrades>,
    query: Query<&Transform, With<Player>>,
) {
    if !run_state.is_running() {
        return;
    }

    let Ok(transform) = query.get_single() else {
        return;
    };
    if !timer.timer.tick(time.delta()).just_finished() {
        return;
    }

    commands.spawn((
        SpriteBundle {
            texture: assets.trail_segment.clone(),
            transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 0.2),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(12.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        TrailSegment {
            remaining: TRAIL_LIFETIME,
            damage: TRAIL_START_DAMAGE as f32 * upgrades.trail_damage_multiplier(),
        },
    ));
}

fn update_trail_segments(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TrailSegment, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut segment, mut transform, mut sprite) in &mut query {
        segment.remaining -= time.delta_seconds();
        if segment.remaining <= 0.0 {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        let t = (segment.remaining / TRAIL_LIFETIME).clamp(0.0, 1.0);
        sprite.color.set_alpha(0.15 + 0.8 * t);
        transform.scale = Vec3::splat(0.8 + t * 0.3);
    }
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    run_state: Res<RunState>,
    score: Res<Score>,
    assets: Res<GameAssets>,
    player_q: Query<&Transform, With<Player>>,
) {
    if !run_state.is_running() {
        return;
    }

    if !timer.timer.tick(time.delta()).just_finished() {
        return;
    }

    let Ok(player_transform) = player_q.get_single() else {
        return;
    };
    let base_pos = player_transform.translation.truncate();

    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..TAU);
    let distance = ENEMY_SPAWN_DISTANCE + rng.gen_range(-100.0..100.0);
    let spawn = base_pos + Vec2::from_angle(angle) * distance;

    let speed = ENEMY_BASE_SPEED + ENEMY_SPEED_INCREMENT * (score.current as f32 / 150.0);
    let health = ENEMY_BASE_HEALTH as f32 + score.current as f32 / 500.0;

    commands.spawn((
        SpriteBundle {
            texture: assets.enemy.clone(),
            transform: Transform::from_xyz(spawn.x, spawn.y, 0.3),
            sprite: Sprite {
                custom_size: Some(Vec2::new(48.0, 44.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Enemy { speed },
        EnemyHealth { current: health },
        EnemyVelocity::default(),
        Knockback::default(),
    ));

    let current = timer.timer.duration().as_secs_f32();
    let new_duration = (current * ENEMY_SPAWN_ACCELERATION).max(0.6);
    timer
        .timer
        .set_duration(Duration::from_secs_f32(new_duration));
}

fn move_enemies(
    time: Res<Time>,
    run_state: Res<RunState>,
    player_q: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies: Query<(
        &Enemy,
        &mut Transform,
        &mut EnemyVelocity,
        &mut Knockback,
        &mut EnemyHealth,
    )>,
) {
    if !run_state.is_running() {
        return;
    }

    let Ok(player_transform) = player_q.get_single() else {
        return;
    };
    let player_position = player_transform.translation.truncate();

    for (enemy, mut transform, mut velocity, mut knockback, mut health) in &mut enemies {
        if health.current <= 0.0 {
            transform.translation.z = -999.0;
            continue;
        }

        let direction = player_position - transform.translation.truncate();
        if direction.length_squared() > 0.1 {
            let desired = direction.normalize() * enemy.speed;
            velocity.current = velocity.current.lerp(desired, ENEMY_TURN_SPEED);
        }

        velocity.current += knockback.velocity;
        knockback.velocity *= 0.92;

        transform.translation += velocity.current.extend(0.0) * time.delta_seconds();
    }
}

fn resolve_trail_hits(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut currency: ResMut<Currency>,
    assets: Res<GameAssets>,
    mut enemies: Query<(Entity, &mut EnemyHealth, &Transform), With<Enemy>>,
    trails: Query<(&TrailSegment, &Transform)>,
) {
    for (enemy_entity, mut health, enemy_transform) in &mut enemies {
        let enemy_pos = enemy_transform.translation.truncate();
        for (trail, trail_transform) in &trails {
            if trail.remaining <= 0.0 {
                continue;
            }
            let trail_pos = trail_transform.translation.truncate();
            if enemy_pos.distance_squared(trail_pos) <= TRAIL_HIT_RADIUS.powi(2) {
                health.current -= trail.damage;
                if health.current <= 0.0 {
                    commands.entity(enemy_entity).despawn_recursive();
                    score.add(BASE_SCORE);
                    currency.credit(1);
                    if random::<f32>() < POWER_UP_DROP_CHANCE {
                        spawn_powerup(
                            &mut commands,
                            &assets,
                            &trail_transform.translation.truncate(),
                        );
                    }
                }
                break;
            }
        }
    }
}

fn spawn_powerup(commands: &mut Commands, assets: &Res<GameAssets>, position: &Vec2) {
    let mut rng = rand::thread_rng();
    let roll = rng.gen::<f32>();
    let cumulative = [
        (PowerUpKind::Currency, POWER_UP_CURRENCY_WEIGHT),
        (PowerUpKind::Health, POWER_UP_HEART_WEIGHT),
        (PowerUpKind::Shield, POWER_UP_SHIELD_WEIGHT),
        (PowerUpKind::Accuracy, POWER_UP_ACCURACY_WEIGHT),
        (PowerUpKind::WaveBlast, POWER_UP_WAVEBLAST_WEIGHT),
    ];

    let mut total = 0.0;
    let mut selected = PowerUpKind::Currency;
    for (kind, weight) in cumulative {
        total += weight;
        if roll <= total {
            selected = kind;
            break;
        }
    }

    let texture = match selected {
        PowerUpKind::Currency => assets.powerup_currency.clone(),
        PowerUpKind::Health => assets.powerup_health.clone(),
        PowerUpKind::Shield => assets.powerup_shield.clone(),
        PowerUpKind::Accuracy => assets.powerup_accuracy.clone(),
        PowerUpKind::WaveBlast => assets.powerup_waveblast.clone(),
    };

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(position.x, position.y, 0.4),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(24.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        PowerUp { kind: selected },
        PowerUpLifetime {
            timer: Timer::from_seconds(POWER_UP_LIFETIME, TimerMode::Once),
        },
    ));
}

fn resolve_player_collisions(
    mut commands: Commands,
    mut run_state: ResMut<RunState>,
    mut player_health: ResMut<PlayerHealth>,
    mut shield: ResMut<ShieldState>,
    mut score: ResMut<Score>,
    mut player_query: Query<(&Transform, &mut Knockback), With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut hit_events: EventWriter<PlayerHitEvent>,
) {
    if !run_state.is_running() {
        return;
    }

    let Ok((player_transform, mut knockback)) = player_query.get_single_mut() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    for (entity, transform) in &enemies {
        let enemy_pos = transform.translation.truncate();
        if enemy_pos.distance_squared(player_pos) <= (PLAYER_RADIUS.powi(2)) as f32 {
            commands.entity(entity).despawn_recursive();
            if shield.is_active() {
                continue;
            }

            player_health.damage(PLAYER_COLLISION_DAMAGE);
            knockback.velocity =
                (player_pos - enemy_pos).normalize_or_zero() * PLAYER_KNOCKBACK_STRENGTH;
            score.current = score.current.saturating_sub(BASE_SCORE / 2);
            hit_events.send(PlayerHitEvent);

            if player_health.is_dead() {
                run_state.end();
            }
        }
    }
}

fn tick_powerups(
    mut commands: Commands,
    time: Res<Time>,
    mut powerups: Query<(Entity, &mut PowerUpLifetime)>,
) {
    for (entity, mut lifetime) in &mut powerups {
        if lifetime.timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn apply_powerup_pickups(
    mut commands: Commands,
    mut currency: ResMut<Currency>,
    mut player_health: ResMut<PlayerHealth>,
    mut shield: ResMut<ShieldState>,
    mut stats: ResMut<PlayerStats>,
    mut upgrades: ResMut<PurchasedUpgrades>,
    player_q: Query<&Transform, With<Player>>,
    mut powerups: Query<(Entity, &PowerUp, &Transform)>,
) {
    let Ok(player_transform) = player_q.get_single() else {
        return;
    };
    let player_pos = player_transform.translation.truncate();

    for (entity, powerup, transform) in &mut powerups {
        let diff = transform.translation.truncate() - player_pos;
        if diff.length_squared() <= (PLAYER_RADIUS * PLAYER_RADIUS) as f32 {
            commands.entity(entity).despawn_recursive();
            match powerup.kind {
                PowerUpKind::Currency => currency.credit(5),
                PowerUpKind::Health => player_health.heal(1),
                PowerUpKind::Shield => {
                    shield.remaining = SHIELD_DURATION + upgrades.shield_duration_bonus();
                    shield.duration = SHIELD_DURATION + upgrades.shield_duration_bonus();
                }
                PowerUpKind::Accuracy => stats.acceleration += 0.02,
                PowerUpKind::WaveBlast => upgrades.trail_damage_level += 1,
            }
        }
    }
}

fn update_shield_state(time: Res<Time>, mut shield: ResMut<ShieldState>) {
    if shield.remaining > 0.0 {
        shield.remaining = (shield.remaining - time.delta_seconds()).max(0.0);
    }
}

fn advance_wave_projectile_timer(
    time: Res<Time>,
    run_state: Res<RunState>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut player_query: Query<(&Transform, &Player, &PlayerVelocity)>,
) {
    if !run_state.is_running() {
        return;
    }

    let Ok((transform, player, velocity)) = player_query.get_single_mut() else {
        return;
    };
    if player.wave_cooldown > 0.0 || velocity.current.length_squared() < 1200.0 {
        return;
    }

    let forward = velocity.current.normalize_or_zero();
    if forward == Vec2::ZERO {
        return;
    }

    let spawn_position = transform.translation.truncate() + forward * 24.0;

    commands.spawn((
        SpriteBundle {
            texture: assets.wave_projectile.clone(),
            transform: Transform::from_xyz(spawn_position.x, spawn_position.y, 0.35),
            sprite: Sprite {
                custom_size: Some(Vec2::new(32.0, 16.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        WaveProjectile {
            velocity: forward * WAVE_SPEED,
            age: 0.0,
            lifetime: WAVE_LIFETIME,
            damage: WAVE_DAMAGE,
        },
    ));
}

fn update_wave_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(Entity, &mut Transform, &mut WaveProjectile)>,
) {
    for (entity, mut transform, mut projectile) in &mut projectiles {
        projectile.age += time.delta_seconds();
        if projectile.age >= projectile.lifetime {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        transform.translation += projectile.velocity.extend(0.0) * time.delta_seconds();

        // TODO: Add collision detection back
        // For now, projectiles just expire naturally after their lifetime
    }
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Transform, &mut Sprite, &mut Particle)>,
) {
    for (entity, mut transform, mut sprite, mut particle) in &mut particles {
        particle.age += time.delta_seconds();
        if particle.age >= particle.lifetime {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        transform.translation += particle.velocity.extend(0.0) * time.delta_seconds();
        sprite
            .color
            .set_alpha(1.0 - particle.age / particle.lifetime);
    }
}

fn apply_screen_shake(
    mut cameras: Query<&mut Transform, With<MainCamera>>,
    mut shake: Query<&mut ScreenShake>,
) {
    let Ok(mut transform) = cameras.get_single_mut() else {
        return;
    };
    let Ok(mut screen_shake) = shake.get_single_mut() else {
        return;
    };

    if screen_shake.trauma > 0.0 {
        let mut rng = rand::thread_rng();
        let offset = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
            * screen_shake.trauma
            * 6.0;
        transform.translation.x = offset.x;
        transform.translation.y = offset.y;
        screen_shake.trauma = (screen_shake.trauma - screen_shake.decay * 0.016).max(0.0);
    } else {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
    }
}

fn handle_player_hit_events(
    mut events: EventReader<PlayerHitEvent>,
    mut shake: Query<&mut ScreenShake>,
) {
    if events.read().next().is_some() {
        if let Ok(mut screen_shake) = shake.get_single_mut() {
            screen_shake.trauma = (screen_shake.trauma + 0.4).min(1.0);
        }
    }
}

fn persist_currency_changes(currency: Res<Currency>) {
    if currency.is_changed() {
        save_currency(currency.balance);
    }
}

fn persist_upgrade_changes(upgrades: Res<PurchasedUpgrades>) {
    if upgrades.is_changed() {
        save_upgrades(&upgrades);
    }
}

fn reset_when_run_stops(
    mut run_state: ResMut<RunState>,
    mut score: ResMut<Score>,
    mut health: ResMut<PlayerHealth>,
) {
    if !run_state.is_active() {
        score.reset_run();
        health.reset();
        run_state.reset();
    }
}

fn apply_shop_purchases(
    mut events: EventReader<ShopPurchaseEvent>,
    mut upgrades: ResMut<PurchasedUpgrades>,
    mut currency: ResMut<Currency>,
    mut health: ResMut<PlayerHealth>,
    mut shield: ResMut<ShieldState>,
) {
    for event in events.read() {
        let Some(item) = SHOP_ITEMS.iter().find(|item| item.upgrade == event.item) else {
            continue;
        };
        let level = match event.item {
            UpgradeType::MovementSpeed => upgrades.movement_speed_level,
            UpgradeType::MaxHealth => upgrades.max_health_level,
            UpgradeType::TrailDamage => upgrades.trail_damage_level,
            UpgradeType::ShieldDuration => upgrades.shield_level,
        };

        if level >= item.max_level {
            continue;
        }

        let cost = item.cost_for_level(level);
        if currency.balance < cost {
            continue;
        }

        currency.balance -= cost;

        match event.item {
            UpgradeType::MovementSpeed => {
                upgrades.movement_speed_level += 1;
            }
            UpgradeType::MaxHealth => {
                upgrades.max_health_level += 1;
                health.max += 1;
                health.current = health.max;
            }
            UpgradeType::TrailDamage => {
                upgrades.trail_damage_level += 1;
            }
            UpgradeType::ShieldDuration => {
                upgrades.shield_level += 1;
                shield.duration += 0.75;
                shield.remaining = shield.duration;
            }
        }
    }
}

fn spawn_background_tiles(commands: &mut Commands, background_texture: Handle<Image>) {
    use threadweaver_core::constants::ARENA_SIZE;

    // Tile size for background (fits within WebGPU limits)
    const TILE_SIZE: f32 = 512.0;
    // Calculate how many tiles we need to cover the arena
    let tiles_per_side = ((ARENA_SIZE / TILE_SIZE).ceil() as i32) + 1;
    let total_tiles = tiles_per_side * tiles_per_side;

    info!("Spawning {} background tiles ({}/side)", total_tiles, tiles_per_side);

    // Calculate starting position to center the grid
    let start_x = -(tiles_per_side as f32 / 2.0) * TILE_SIZE;
    let start_y = -(tiles_per_side as f32 / 2.0) * TILE_SIZE;

    for y in 0..tiles_per_side {
        for x in 0..tiles_per_side {
            let pos_x = start_x + (x as f32) * TILE_SIZE;
            let pos_y = start_y + (y as f32) * TILE_SIZE;

            commands.spawn((
                SpriteBundle {
                    texture: background_texture.clone(),
                    transform: Transform::from_xyz(pos_x, pos_y, -100.0),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Background,
            ));
        }
    }
}
