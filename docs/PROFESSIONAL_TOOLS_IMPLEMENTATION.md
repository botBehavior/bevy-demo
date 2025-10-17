# Professional Tools Implementation Plan
## Avian2D Physics + bevy_hanabi Particles

**Date**: October 17, 2025  
**Approach**: Option B - Best Quality Game  
**Priority Order**: UX Fixes → Professional Tools

---

## 🎯 Implementation Order

### Phase 0: Critical UX Fixes (MUST DO FIRST)
1. ✅ **Fix cursor lock on web** - Game unplayable without this
2. ✅ **Increase canvas size** - Better visibility and tablet compatibility  
3. ✅ **Fix HUD font** - Need to see game state

### Phase 1: Professional Physics
4. ✅ Add Avian2D dependency
5. ✅ Refactor player with physics
6. ✅ Refactor enemies with physics

### Phase 2: Professional Particles
7. ✅ Add bevy_hanabi dependency
8. ✅ Create death explosion effects
9. ✅ Create pickup effects
10. ✅ Add atmospheric particles

---

## 🚨 Phase 0: Critical UX Fixes

### Fix 1: Cursor Lock (CRITICAL FOR PLAYABILITY)

**Problem**: 
- Cursor escapes canvas on web
- Pointer lock API not working correctly
- ESC should pause and unlock cursor

**Root Cause Analysis**:
Looking at current code (lines 1010-1032), the `enforce_cursor_lock` function exists but may not work properly because:
1. Bevy's `CursorGrabMode::Locked` requires user gesture on web
2. Need to request pointer lock on first click
3. ESC handling conflicts with browser behavior

**Solution**:

```rust
// Update cursor lock to work on web
#[derive(Resource, Default)]
struct CursorLockState {
    locked: bool,
    requested: bool,
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

    // On web, we need user interaction to lock cursor
    // Request lock on first mouse click
    #[cfg(target_arch = "wasm32")]
    if !state.requested && mouse_button.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        state.requested = true;
        state.locked = true;
        return;
    }

    if run_state.is_running() {
        if !state.locked {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
            state.locked = true;
        }
    } else {
        // Paused or game over - unlock cursor
        if state.locked {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
            state.locked = false;
        }
    }
}

// Update pause handler to properly unlock
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
```

**Testing**:
- [ ] Click canvas → cursor locks
- [ ] Move mouse → stays in canvas
- [ ] Press ESC → cursor unlocks, game pauses
- [ ] Press ESC again → cursor locks, game resumes
- [ ] Die → cursor unlocks
- [ ] Press SPACE → cursor locks on resume

---

### Fix 2: Canvas Size (TABLET COMPATIBILITY)

**Current Size**: 960x720 (too small)  
**Target Size**: 1024x768 (standard tablet landscape)  
**Alternative**: 1280x720 (16:9 widescreen)

**Files to Update**:

**1. src/main.rs - Update constants:**
```rust
// OLD:
const ARENA_BOUNDS: Vec2 = Vec2::new(960.0, 720.0);

// NEW - Tablet size:
const ARENA_BOUNDS: Vec2 = Vec2::new(1024.0, 768.0);

// OR - Widescreen:
const ARENA_BOUNDS: Vec2 = Vec2::new(1280.0, 720.0);
```

**2. index.html - Update canvas CSS:**
```html
<!-- OLD: -->
<style>
  canvas {
    width: min(100vw, 960px);
    height: min(100vh, 720px);
    outline: none;
    touch-action: none;
  }
</style>

<!-- NEW - Tablet: -->
<style>
  canvas {
    width: min(100vw, 1024px);
    height: min(100vh, 768px);
    outline: none;
    touch-action: none;
  }
</style>

<!-- OR - Widescreen: -->
<style>
  canvas {
    width: min(100vw, 1280px);
    height: min(100vh, 720px);
    outline: none;
    touch-action: none;
  }
</style>
```

**Recommendation**: Go with **1024x768** because:
- Standard tablet resolution (iPad, etc.)
- 4:3 aspect ratio = more vertical space for gameplay
- Works well on desktop and tablets
- Not too large for loading/performance

---

### Fix 3: HUD Font (CRITICAL FOR VISIBILITY)

**Quick Implementation**:

1. **Get font file** (I'll download for you):
   - Orbitron-Bold.ttf (sci-fi theme, perfect for Threadweaver)
   - OR FiraSans-Bold.ttf (readable, professional)

2. **Update Cargo.toml** to embed font:
```toml
# Add to enable embedded assets in release builds
[dependencies]
bevy = { version = "0.14", default-features = false, features = [
    "bevy_asset",
    "bevy_core_pipeline",
    "bevy_render",
    "bevy_sprite",
    "bevy_state",
    "bevy_text",
    "bevy_ui",
    "bevy_winit",
    "png",
    "embedded_watcher",  # NEW - helps with font loading
] }
```

3. **Update setup() function**:
```rust
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,  // NEW parameter
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    
    // Load font
    let font = asset_server.load("fonts/Orbitron-Bold.ttf");
    
    // ... (borders and player spawn) ...
    
    // HUD Container with FONT
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
            // Score
            parent.spawn((
                TextBundle::from_section(
                    "Score: 0 | Best: 0",
                    TextStyle {
                        font: font.clone(),  // ← ADD THIS
                        font_size: 22.0,
                        color: Color::WHITE,
                    },
                ),
                HudScore,
            ));

            // Health
            parent.spawn((
                TextBundle::from_section(
                    format!("Health: {}/{}", PLAYER_MAX_HEALTH, PLAYER_MAX_HEALTH),
                    TextStyle {
                        font: font.clone(),  // ← ADD THIS
                        font_size: 20.0,
                        color: Color::srgb(1.0, 0.6, 0.7),
                    },
                ),
                HudHealth,
            ));

            // Combo
            parent.spawn((
                TextBundle::from_section(
                    "Combo x1.0 (0)",
                    TextStyle {
                        font: font.clone(),  // ← ADD THIS
                        font_size: 18.0,
                        color: Color::srgb(0.7, 0.9, 1.0),
                    },
                ),
                HudCombo,
            ));

            // Buffs
            parent.spawn((
                TextBundle::from_section(
                    "Damage x1.0 | Shield: Ready",
                    TextStyle {
                        font: font.clone(),  // ← ADD THIS
                        font_size: 18.0,
                        color: Color::srgb(0.8, 0.8, 0.9),
                    },
                ),
                HudBuffs,
            ));

            // Status
            parent.spawn((
                TextBundle::from_section(
                    "Status: Running",
                    TextStyle {
                        font: font.clone(),  // ← ADD THIS
                        font_size: 18.0,
                        color: Color::srgb(0.6, 0.6, 0.6),
                    },
                ),
                HudStatus,
            ));
        });
}
```

---

## 🎮 Phase 1: Avian2D Physics

### Step 1: Add Dependencies

**Update Cargo.toml:**
```toml
[dependencies]
bevy = { version = "0.14", default-features = false, features = [
    "bevy_asset",
    "bevy_core_pipeline",
    "bevy_render",
    "bevy_sprite",
    "bevy_state",
    "bevy_text",
    "bevy_ui",
    "bevy_winit",
    "png",
] }
log = "0.4"
rand = "0.8"
avian2d = "0.1"  # NEW - Professional 2D physics

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
bevy = { version = "0.14", default-features = false, features = ["x11"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
bevy = { version = "0.14", default-features = false, features = ["webgl2"] }
avian2d = { version = "0.1", default-features = false, features = ["2d", "f32"] }  # WASM optimized
getrandom = { version = "0.3", features = ["std", "wasm_js"] }
uuid = { version = "1", features = ["std", "rng-getrandom"] }
console_error_panic_hook = "0.1"
```

### Step 2: Initialize Physics Plugin

```rust
use avian2d::prelude::*;

fn main() {
    // ...
    
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        // ... existing resources ...
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
        .add_plugins(PhysicsPlugins::default())  // NEW - Physics engine
        .insert_resource(Gravity(Vec2::ZERO))    // NEW - No gravity (top-down game)
        // ... rest of setup
}
```

### Step 3: Refactor Player with Physics

```rust
// Update Player component
#[derive(Component)]
struct Player {
    target_velocity: Vec2,
}

// Spawn player with physics
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ... camera and borders ...
    
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
        Player {
            target_velocity: Vec2::ZERO,
        },
        // Physics components
        RigidBody::Kinematic,  // Player-controlled (not affected by forces)
        Collider::circle(PLAYER_RADIUS),
        LockedAxes::ROTATION_LOCKED,  // Don't rotate
    ));
    
    // ... HUD setup ...
}

// Update movement system
fn move_player(
    time: Res<Time>,
    run_state: Res<RunState>,
    target: Res<PointerTarget>,
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    if !run_state.is_running() {
        return;
    }

    let (mut transform, mut player) = query.single_mut();
    let current = transform.translation.truncate();
    let delta = target.position - current;
    let distance = delta.length();

    if distance > f32::EPSILON {
        // Smooth acceleration
        let desired = delta.normalize() * PLAYER_SPEED;
        player.target_velocity = player.target_velocity.lerp(desired, 0.15);
    } else {
        // Deceleration
        player.target_velocity = player.target_velocity.lerp(Vec2::ZERO, 0.3);
    }

    // Apply movement
    transform.translation += player.target_velocity.extend(0.0) * time.delta_seconds();
    clamp_to_bounds(&mut transform.translation);
}
```

### Step 4: Refactor Enemies with Physics

```rust
// Update Enemy component
#[derive(Component)]
struct Enemy {
    speed: f32,
    steering_force: Vec2,
}

// Spawn enemies with physics
fn spawn_enemies(
    mut commands: Commands,
    // ... existing params ...
) {
    // ... spawn timing logic ...
    
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
        Enemy {
            speed: enemy_speed,
            steering_force: Vec2::ZERO,
        },
        EnemyHealth {
            current: ENEMY_BASE_HEALTH as f32,
        },
        // Physics components
        RigidBody::Dynamic,  // Affected by forces
        Collider::rectangle(ENEMY_SIZE.x, ENEMY_SIZE.y),
        LinearVelocity::default(),
        LockedAxes::ROTATION_LOCKED,
        Mass::new(1.0),
    ));
}

// Update enemy movement with steering
fn move_enemies(
    time: Res<Time>,
    run_state: Res<RunState>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemies: Query<(&Enemy, &Transform, &mut LinearVelocity), Without<Player>>,
) {
    if !run_state.is_running() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    
    for (enemy, transform, mut velocity) in &mut enemies {
        let pos = transform.translation.truncate();
        
        // Steering behavior
        let desired = (player_pos - pos).normalize() * enemy.speed;
        let steering = desired - velocity.0;
        let steering_force = steering.clamp_length_max(enemy.speed * 0.15);
        
        // Apply steering
        velocity.0 += steering_force * time.delta_seconds() * 50.0;
        velocity.0 = velocity.0.clamp_length_max(enemy.speed);
    }
}
```

### Step 5: Update Collision Detection

```rust
// Use Avian's collision events
fn handle_player_collisions(
    mut commands: Commands,
    mut run_state: ResMut<RunState>,
    mut score: ResMut<Score>,
    mut combo: ResMut<Combo>,
    mut health: ResMut<PlayerHealth>,
    shield: Res<ShieldState>,
    mut collision_events: EventReader<CollisionStarted>,
    player_query: Query<Entity, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
) {
    if !run_state.is_running() {
        return;
    }

    let player_entity = player_query.single();

    for CollisionStarted(entity1, entity2) in collision_events.read() {
        // Check if player collided with enemy
        let enemy_entity = if *entity1 == player_entity && enemies.contains(*entity2) {
            Some(*entity2)
        } else if *entity2 == player_entity && enemies.contains(*entity1) {
            Some(*entity1)
        } else {
            None
        };

        if let Some(enemy) = enemy_entity {
            commands.entity(enemy).despawn_recursive();
            
            if !shield.is_active() {
                health.apply_damage(PLAYER_COLLISION_DAMAGE);
                if health.current == 0 {
                    run_state.active = false;
                    score.best = score.best.max(score.current);
                    combo.reset();
                }
            }
            break;
        }
    }
}
```

---

## ✨ Phase 2: bevy_hanabi Particles

### Step 1: Add Dependencies

**Update Cargo.toml:**
```toml
[dependencies]
# ... existing deps ...
bevy_hanabi = "0.12"  # NEW - GPU particle system

[target.'cfg(target_arch = "wasm32")'.dependencies]
# ... existing deps ...
bevy_hanabi = { version = "0.12", default-features = false }  # WASM version
```

### Step 2: Initialize Hanabi Plugin

```rust
use bevy_hanabi::prelude::*;

fn main() {
    // ...
    
    app
        // ... existing plugins ...
        .add_plugins(HanabiPlugin)  // NEW - Particle system
        // ...
}
```

### Step 3: Create Death Explosion Effect

```rust
// Setup particle effect assets
fn setup_particle_effects(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    // Death explosion effect
    let death_effect = effects.add(
        EffectAsset::new(
            vec![512],  // Max 512 particles
            Spawner::burst(200.0.into(), 0.1.into()),  // Burst 200 particles over 0.1s
            Module::default(),
        )
        .with_name("death_explosion")
        .init(PositionCircleModifier {
            center: Vec3::ZERO.into(),
            radius: 5.0.into(),
            dimension: ShapeDimension::Surface,
        })
        .init(ParticleLifetimeModifier { lifetime: 0.8 })
        .update(LinearDragModifier { drag: 2.0 })
        .render(ColorOverLifetimeModifier {
            gradient: Gradient::linear(
                Vec4::new(1.0, 0.5, 0.5, 1.0),  // Red
                Vec4::new(1.0, 0.3, 0.0, 0.0),  // Orange fade to transparent
            ),
        })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::linear(
                Vec2::splat(8.0),   // Start size
                Vec2::splat(0.0),   // End size
            ),
        }),
    );

    commands.insert_resource(DeathExplosionEffect(death_effect));
}

#[derive(Resource)]
struct DeathExplosionEffect(Handle<EffectAsset>);

// Spawn explosion on enemy death
fn spawn_death_explosion(
    commands: &mut Commands,
    position: Vec2,
    effect_handle: &Handle<EffectAsset>,
) {
    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle.clone()),
            transform: Transform::from_translation(position.extend(0.3)),
            ..default()
        },
        DespawnAfter(Timer::from_seconds(1.0, TimerMode::Once)),
    ));
}

#[derive(Component)]
struct DespawnAfter(Timer);

fn despawn_after_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DespawnAfter)>,
) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// Hook into enemy defeat
fn handle_trail_collisions(
    mut commands: Commands,
    // ... existing params ...
    death_effect: Res<DeathExplosionEffect>,
) {
    // ... existing logic ...
    
    if !defeated.is_empty() {
        for (entity, position) in defeated {
            score.current += combo.register_kill();
            spawn_death_explosion(&mut commands, position, &death_effect.0);  // NEW
            maybe_spawn_power_up(&mut commands, rng, position);
            commands.entity(entity).despawn_recursive();
        }
    }
}
```

### Step 4: Power-Up Pickup Effects

```rust
// Setup pickup effect
fn setup_pickup_effects(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let pickup_effect = effects.add(
        EffectAsset::new(
            vec![64],
            Spawner::burst(16.0.into(), 0.05.into()),
            Module::default(),
        )
        .with_name("pickup_ring")
        .init(PositionCircleModifier {
            center: Vec3::ZERO.into(),
            radius: 20.0.into(),
            dimension: ShapeDimension::Surface,
        })
        .init(VelocityCircleModifier {
            center: Vec3::ZERO.into(),
            speed: 80.0.into(),
        })
        .init(ParticleLifetimeModifier { lifetime: 0.5 })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::linear(Vec2::splat(6.0), Vec2::splat(0.0)),
        }),
    );

    commands.insert_resource(PickupEffect(pickup_effect));
}

#[derive(Resource)]
struct PickupEffect(Handle<EffectAsset>);

fn spawn_pickup_effect(
    commands: &mut Commands,
    position: Vec2,
    color: Color,
    effect_handle: &Handle<EffectAsset>,
) {
    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle.clone())
                .with_property("color", color.to_linear().to_vec4().into()),
            transform: Transform::from_translation(position.extend(0.3)),
            ..default()
        },
        DespawnAfter(Timer::from_seconds(0.6, TimerMode::Once)),
    ));
}
```

---

## 📋 Complete Implementation Checklist

### Phase 0: Critical Fixes
- [ ] Download Orbitron-Bold.ttf to `assets/fonts/`
- [ ] Update setup() to load font with AssetServer
- [ ] Apply font to all 5 HUD text elements
- [ ] Test HUD visibility
- [ ] Update ARENA_BOUNDS to 1024x768
- [ ] Update index.html canvas CSS
- [ ] Test canvas size
- [ ] Fix cursor lock with mouse button detection
- [ ] Update pause handler to unlock cursor
- [ ] Test cursor lock: click → lock, ESC → unlock

### Phase 1: Avian Physics  
- [ ] Add avian2d to Cargo.toml
- [ ] Add PhysicsPlugins to app
- [ ] Set Gravity to zero
- [ ] Refactor Player with Kinematic RigidBody
- [ ] Add Collider to player
- [ ] Update move_player with smooth acceleration
- [ ] Test player movement feel
- [ ] Refactor Enemy with Dynamic RigidBody
- [ ] Add steering behavior to enemies
- [ ] Update collision detection with CollisionStarted events
- [ ] Test enemy behavior

### Phase 2: Hanabi Particles
- [ ] Add bevy_hanabi to Cargo.toml
- [ ] Add HanabiPlugin to app
- [ ] Create death explosion effect asset
- [ ] Hook death explosion into enemy defeat
- [ ] Test death explosions look good
- [ ] Create pickup ring effect asset
- [ ] Hook pickup effects into power-up collection
- [ ] Test pickup effects
- [ ] Add despawn_after_timer system
- [ ] Performance test with many particles

---

## 🎯 Expected Results

After all implementations:

✅ **UX Fixes**:
- HUD clearly visible with sci-fi font
- Canvas size 1024x768 (tablet-friendly)
- Cursor locks on click, unlocks on ESC
- Game actually playable

✅ **Physics**:
- Player movement has satisfying momentum
- Enemies curve naturally toward player
- Collisions feel solid and responsive
- 60 FPS with 50+ enemies

✅ **Particles**:
- Death explosions look AAA quality
- Pickup effects clearly visible
- GPU-accelerated for performance
- Can add atmospheric effects easily

✅ **Bundle**:
- ~3.1 MB total (very reasonable)
- Loads in 1-2 seconds on 4G
- Professional game quality

---

## 🚀 Let's Start!

Ready to implement? I recommend this order:

1. **First**: Fix cursor lock (makes game playable)
2. **Second**: Increase canvas size (better experience)
3. **Third**: Fix HUD font (see what's happening)
4. **Then**: Add Avian physics (professional feel)
5. **Finally**: Add Hanabi particles (visual wow factor)

Should I start implementing these changes?

