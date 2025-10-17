# In-Depth Research Findings: Bevy 0.14 Improvements
## Comprehensive Analysis for HUD, Physics, and Particles

**Date**: October 17, 2025  
**Bevy Version**: 0.14.2  
**Project**: Threadweaver MVP

---

## Table of Contents
1. [HUD Visibility Issue](#1-hud-visibility-issue)
2. [Physics System Research](#2-physics-system-research)
3. [Particle System Options](#3-particle-system-options)
4. [Final Implementation Plan](#4-final-implementation-plan)

---

## 1. HUD Visibility Issue

### Problem Analysis

**Root Cause**: Bevy 0.14 does not include a default embedded font. When `TextBundle` is created with `TextStyle::default()` or without explicitly loading a font asset, the text simply will not render. This is a **critical blocker** for the MVP.

### Research Findings

#### A. Font Rendering in Bevy 0.14

**How Bevy Handles Text**:
- Bevy's `bevy_text` module requires an explicit `Handle<Font>` in `TextStyle`
- Font assets must be loaded through `AssetServer` before use
- The asset system is asynchronous - fonts load in the background
- No fallback font is provided in minimal builds (which we're using)

**Current Issue in Our Code** (lines 452-456):
```rust
TextStyle {
    font_size: 22.0,
    color: Color::WHITE,
    ..Default::default()  // ❌ This provides no font handle!
}
```

#### B. Solutions Evaluated

**Option 1: Load Custom TTF Font** ⭐ **RECOMMENDED**
```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font: font.clone(),  // ✅ Explicit font reference
                font_size: 22.0,
                color: Color::WHITE,
            },
        ),
        HudScore,
    ));
}
```

**Pros**:
- Full control over typography
- Works on all platforms (native + WASM)
- Small file size (~50-100KB for typical font)
- Professional appearance

**Cons**:
- Requires font file asset
- Need to ensure font license allows embedding

**Best Fonts for Web Games**:
1. **FiraSans** - Open source, excellent readability
2. **Roboto** - Google's font, very popular
3. **Source Code Pro** - For monospace/tech aesthetic
4. **Orbitron** - Futuristic/sci-fi vibe (fits Threadweaver theme)

**Implementation Checklist**:
- [ ] Create `assets/fonts/` directory
- [ ] Download/add font file (e.g., `FiraSans-Bold.ttf`)
- [ ] Modify `setup()` to accept `AssetServer`
- [ ] Load font: `let font = asset_server.load("fonts/FiraSans-Bold.ttf")`
- [ ] Apply to all `TextStyle` instances in HUD
- [ ] Test on both native and WASM builds

---

**Option 2: Use Minimal System Font Access** ⚠️ **NOT RECOMMENDED**
- Bevy 0.14 doesn't have built-in system font loading
- Would require additional crate like `font-kit`
- Inconsistent across platforms
- Adds complexity without benefits

---

**Option 3: HTML Overlay (Web-Only)** ⚠️ **BREAKS PORTABILITY**
- Use HTML/CSS for HUD instead of Bevy UI
- Only works in browser
- Requires JavaScript interop
- Defeats purpose of having unified codebase

---

### Performance Considerations

| Approach | WASM Size | Load Time | Render Cost |
|----------|-----------|-----------|-------------|
| Custom TTF Font | +50-100KB | ~50ms | Minimal |
| System Font | +200KB (font-kit) | Variable | Minimal |
| HTML Overlay | 0KB (Bevy) | Instant | N/A |

**Verdict**: Custom TTF font is best balance of size, performance, and portability.

---

## 2. Physics System Research

### Current State Analysis

**What We Have**:
- Simple linear interpolation to cursor target
- No momentum or acceleration
- Basic circle-to-circle collision (radius checks)
- O(n×m) collision detection (enemies × trail segments)
- Enemies move in straight lines toward player

**What We Need**:
- More satisfying movement "feel"
- Better enemy AI behaviors
- Optimized collision detection
- Possible advanced features (bouncing, friction, etc.)

### Physics Options Evaluated

#### Option A: Custom Lightweight Physics ⭐ **RECOMMENDED FOR MVP+**

**Momentum-Based Movement**:
```rust
#[derive(Component)]
struct Velocity {
    linear: Vec2,
}

const ACCELERATION: f32 = 0.15;
const MAX_SPEED: f32 = 900.0;

fn move_player_with_momentum(
    time: Res<Time>,
    target: Res<PointerTarget>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let current = transform.translation.truncate();
    let delta = target.position - current;
    
    if delta.length() > f32::EPSILON {
        let desired_velocity = delta.normalize() * MAX_SPEED;
        // Smooth acceleration
        velocity.linear = velocity.linear.lerp(desired_velocity, ACCELERATION);
    } else {
        // Deceleration when at target
        velocity.linear = velocity.linear.lerp(Vec2::ZERO, ACCELERATION * 2.0);
    }
    
    let movement = velocity.linear * time.delta_seconds();
    transform.translation += movement.extend(0.0);
}
```

**Steering Behaviors for Enemies**:
```rust
fn steering_seek(current_pos: Vec2, target_pos: Vec2, current_vel: Vec2, max_speed: f32) -> Vec2 {
    let desired = (target_pos - current_pos).normalize() * max_speed;
    let steering = desired - current_vel;
    steering.clamp_length_max(max_speed * 0.15) // Max turn rate
}

fn move_enemies_with_steering(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemies: Query<(&mut Transform, &mut Velocity, &Enemy)>,
) {
    let player_pos = player_query.single().translation.truncate();
    
    for (mut transform, mut velocity, enemy) in &mut enemies {
        let pos = transform.translation.truncate();
        let steering = steering_seek(pos, player_pos, velocity.linear, enemy.speed);
        
        velocity.linear += steering;
        velocity.linear = velocity.linear.clamp_length_max(enemy.speed);
        
        transform.translation += velocity.linear.extend(0.0) * time.delta_seconds();
    }
}
```

**Spatial Hash Grid for Collision Optimization**:
```rust
const CELL_SIZE: f32 = 50.0;

struct SpatialHash {
    cells: HashMap<(i32, i32), Vec<Entity>>,
}

impl SpatialHash {
    fn insert(&mut self, entity: Entity, pos: Vec2) {
        let cell = self.pos_to_cell(pos);
        self.cells.entry(cell).or_default().push(entity);
    }
    
    fn pos_to_cell(&self, pos: Vec2) -> (i32, i32) {
        (
            (pos.x / CELL_SIZE).floor() as i32,
            (pos.y / CELL_SIZE).floor() as i32,
        )
    }
    
    fn get_nearby(&self, pos: Vec2) -> Vec<Entity> {
        let center = self.pos_to_cell(pos);
        let mut nearby = Vec::new();
        
        // Check 3x3 grid around position
        for dx in -1..=1 {
            for dy in -1..=1 {
                let cell = (center.0 + dx, center.1 + dy);
                if let Some(entities) = self.cells.get(&cell) {
                    nearby.extend(entities);
                }
            }
        }
        nearby
    }
}
```

**Pros**:
- Zero dependency cost
- Full control over behavior
- Lightweight (~200 lines of code)
- Easy to tune and adjust
- Perfect for our game's needs

**Cons**:
- Need to implement everything ourselves
- No built-in constraint solving
- Limited to simple physics

**WASM Impact**: 0KB (pure Rust code)

---

#### Option B: bevy_rapier2d - Professional Physics Engine

**Latest Stable**: `bevy_rapier2d = "0.27"` (compatible with Bevy 0.14)

**Features**:
- Industry-standard Rapier physics engine
- Rigid body dynamics (dynamic, kinematic, fixed)
- Advanced collision detection with shapes (circles, polygons, compound)
- Contact events and collision filtering
- Joints and constraints
- Continuous collision detection (prevents tunneling)
- Forces, impulses, gravity

**Example Setup**:
```rust
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        RigidBody::KinematicVelocityBased, // Player controlled
        Collider::ball(PLAYER_RADIUS),
        KinematicCharacterController::default(),
        Transform::default(),
        Player,
    ));
}

fn setup_enemy(mut commands: Commands, position: Vec2) {
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(ENEMY_SIZE.x / 2.0, ENEMY_SIZE.y / 2.0),
        Velocity::linear(Vec2::ZERO),
        ExternalForce::default(),
        Transform::from_translation(position.extend(0.0)),
        Enemy { speed: 220.0 },
    ));
}
```

**Pros**:
- Professional-grade physics
- Handles complex scenarios automatically
- Well-documented and maintained
- Large community support
- Debug visualization built-in

**Cons**:
- Adds ~800KB to WASM bundle
- More complex than needed for our game
- Requires learning Rapier's API
- Potential performance overhead for simple collisions

**WASM Impact**: +~800KB to bundle size

**Performance**: Excellent (uses spatial partitioning internally)

**Recommendation**: **Overkill for current needs** - save for future if we add:
- Physics-based power-ups
- Bouncing projectiles
- Destructible environment
- Complex enemy behaviors with forces

---

#### Option C: Avian (formerly bevy_xpbd) - Modern ECS-First Physics

**Latest**: `avian2d = "0.1"` (Bevy 0.14 compatible)

**Background**: 
- XPBD = Extended Position Based Dynamics
- Recently rebranded to "Avian"
- More Bevy-native than Rapier
- Newer, gaining popularity

**Features**:
- Similar to Rapier but more ECS-aligned
- Lighter weight implementation
- Good performance
- Simpler API for Bevy users

**Example**:
```rust
use avian2d::prelude::*;

App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(PhysicsPlugins::default())
    .run();

commands.spawn((
    RigidBody::Dynamic,
    Collider::circle(PLAYER_RADIUS),
    Player,
));
```

**Pros**:
- More Bevy-idiomatic than Rapier
- Lighter bundle size (~500KB)
- Simpler API
- Active development

**Cons**:
- Newer, less battle-tested
- Smaller community
- Still more than we need for MVP

**WASM Impact**: +~500KB

---

### Physics Recommendation Matrix

| Feature Needed | Custom | Rapier | Avian |
|----------------|--------|--------|-------|
| Player momentum | ✅ Easy | ✅ Easy | ✅ Easy |
| Steering behaviors | ✅ Full control | ⚠️ Indirect | ⚠️ Indirect |
| Trail collision | ✅ Optimized | ✅ Auto | ✅ Auto |
| Future expandability | ⚠️ Limited | ✅ Excellent | ✅ Very good |
| Bundle size | ✅ 0KB | ❌ +800KB | ⚠️ +500KB |
| Learning curve | ✅ Simple | ⚠️ Moderate | ✅ Easy |

**Final Verdict**: 
- **MVP+**: Custom lightweight physics (momentum + steering + spatial hash)
- **Future**: Consider Avian if we need advanced features

---

## 3. Particle System Options

### Particle Requirements Analysis

**What We Need Particles For**:
1. Enemy death explosions (HIGH PRIORITY) - immediate feedback
2. Power-up pickup feedback (HIGH PRIORITY) - reward recognition
3. Trail shimmer/glow (MEDIUM PRIORITY) - aesthetic polish
4. Player damage flash (MEDIUM PRIORITY) - danger awareness
5. Combo milestone effects (LOW PRIORITY) - achievement celebration

**Performance Budget**: 100-200 active particles at 60 FPS

### Options Evaluated

#### Option A: Custom Sprite-Based Particles ⭐ **RECOMMENDED FOR MVP+**

**Simple, Effective Implementation**:
```rust
#[derive(Component)]
struct Particle {
    velocity: Vec2,
    lifetime: Timer,
    start_size: f32,
    end_size: f32,
    start_color: Color,
    end_color: Color,
}

impl Particle {
    fn new(velocity: Vec2, lifetime_secs: f32) -> Self {
        Self {
            velocity,
            lifetime: Timer::from_seconds(lifetime_secs, TimerMode::Once),
            start_size: 8.0,
            end_size: 2.0,
            start_color: Color::srgba(1.0, 0.5, 0.5, 1.0),
            end_color: Color::srgba(1.0, 0.5, 0.5, 0.0),
        }
    }
}

fn spawn_death_explosion(
    commands: &mut Commands,
    position: Vec2,
    count: usize,
) {
    use std::f32::consts::TAU;
    
    for i in 0..count {
        let angle = (i as f32 / count as f32) * TAU;
        let speed = 150.0 + rand::random::<f32>() * 100.0;
        let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(1.0, 0.5, 0.5, 0.8),
                    custom_size: Some(Vec2::splat(8.0)),
                    ..default()
                },
                transform: Transform::from_translation(position.extend(0.3)),
                ..default()
            },
            Particle::new(velocity, 0.6),
        ));
    }
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Particle, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut particle, mut transform, mut sprite) in &mut particles {
        particle.lifetime.tick(time.delta());
        
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }
        
        // Update position
        transform.translation += particle.velocity.extend(0.0) * time.delta_seconds();
        
        // Apply drag
        particle.velocity *= 0.95;
        
        // Interpolate size and color
        let t = particle.lifetime.fraction();
        let size = particle.start_size.lerp(particle.end_size, t);
        sprite.custom_size = Some(Vec2::splat(size));
        
        let color = Color::srgba(
            particle.start_color.red().lerp(particle.end_color.red(), t),
            particle.start_color.green().lerp(particle.end_color.green(), t),
            particle.start_color.blue().lerp(particle.end_color.blue(), t),
            particle.start_color.alpha().lerp(particle.end_color.alpha(), t),
        );
        sprite.color = color;
    }
}
```

**Specific Effects to Implement**:

**1. Enemy Death Explosion**:
```rust
spawn_death_explosion(commands, enemy_pos, 12); // 12 particles radiating out
```
- Color: Red→Orange fade
- Duration: 0.6 seconds
- Radial velocity: 150-250 px/s
- Size: 8px → 2px

**2. Power-Up Pickup Ring**:
```rust
fn spawn_pickup_effect(commands: &mut Commands, position: Vec2, power_up_color: Color) {
    for i in 0..8 {
        let angle = (i as f32 / 8.0) * TAU;
        let offset = Vec2::new(angle.cos(), angle.sin()) * 20.0;
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: power_up_color,
                    custom_size: Some(Vec2::splat(6.0)),
                    ..default()
                },
                transform: Transform::from_translation((position + offset).extend(0.3)),
                ..default()
            },
            Particle {
                velocity: offset * 3.0, // Expand outward
                lifetime: Timer::from_seconds(0.4, TimerMode::Once),
                // ...
            },
        ));
    }
}
```

**3. Trail Glow Particles** (subtle):
```rust
// Spawn 1 particle every 5 trail segments for subtle sparkle
if random::<f32>() < 0.2 {
    let drift = Vec2::new(
        (random::<f32>() - 0.5) * 10.0,
        (random::<f32>() - 0.5) * 10.0,
    );
    spawn_trail_sparkle(commands, trail_pos + drift);
}
```

**4. Damage Flash** (screen effect):
```rust
#[derive(Component)]
struct ScreenFlash {
    timer: Timer,
    color: Color,
}

fn spawn_damage_flash(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 0.0, 0.0, 0.3),
                custom_size: Some(ARENA_BOUNDS * 2.0), // Fullscreen
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        ScreenFlash {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
            color: Color::srgba(1.0, 0.0, 0.0, 0.3),
        },
    ));
}
```

**Pros**:
- **Zero external dependencies**
- Full creative control
- Very lightweight
- Easy to debug and tune
- Perfect for our needs

**Cons**:
- Manual implementation required
- CPU-based (but fine for 100-200 particles)
- No fancy GPU compute features

**WASM Impact**: ~5KB of code

**Performance**: 100-200 particles @ 60 FPS easily achievable

---

#### Option B: bevy_hanabi - GPU Particle System

**Latest**: `bevy_hanabi = "0.12"` (Bevy 0.14 compatible)

**What It Is**:
- GPU-accelerated particle system
- Compute shader-based
- Can handle 10,000+ particles
- Visual effect graphs

**Example**:
```rust
use bevy_hanabi::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HanabiPlugin)
        .run();
}

fn setup_explosion_effect(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let effect = effects.add(
        EffectAsset::new(
            vec![256], // Max particles per system
            Spawner::burst(200.0.into(), 2.0.into()), // 200 particles over 2 seconds
            Module::default(),
        )
        .with_name("explosion")
        .init(PositionSphereModifier {
            center: Vec3::ZERO.into(),
            radius: 5.0.into(),
            dimension: ShapeDimension::Surface,
        })
        .init(ParticleLifetimeModifier { lifetime: 1.5 })
        .update(AccelModifier::constant(Vec3::ZERO))
        .render(ColorOverLifetimeModifier {
            gradient: Gradient::linear(
                Vec4::new(1.0, 0.5, 0.0, 1.0),
                Vec4::new(1.0, 0.0, 0.0, 0.0),
            ),
        })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::linear(Vec2::splat(10.0), Vec2::splat(0.0)),
        }),
    );
    
    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
    ));
}
```

**Pros**:
- **GPU-accelerated** - can handle thousands of particles
- Professional visual quality
- Compute shader optimization
- Good for atmospheric effects
- Active maintenance

**Cons**:
- **+500KB WASM bundle**
- Learning curve (effect graph system)
- Overkill for our particle count
- May not work on all WebGL implementations
- More complex debugging

**WASM Impact**: +~500KB

**WASM Compatibility**: ⚠️ Requires WebGL2 with compute shader support (not universal)

**Performance**: Excellent when working, but adds initial load time

---

#### Option C: Hybrid Approach

**Strategy**: 
- Use **custom sprites** for gameplay particles (deaths, pickups, hits)
- Reserve **Hanabi** for background atmosphere only (if needed)

**Example Use Cases**:
- Custom: Enemy deaths, power-ups, damage (critical feedback)
- Hanabi: Ambient floating dust, arena effects (aesthetic only)

**Pros**:
- Best of both worlds
- Critical effects load immediately
- Can add Hanabi later for polish

**Cons**:
- Two systems to manage
- Still adds bundle size

---

### Particle System Recommendation Matrix

| Requirement | Custom Sprites | Hanabi | Hybrid |
|-------------|----------------|--------|--------|
| Enemy deaths (12 particles) | ✅ Perfect | ✅ Overkill | ✅ Good |
| Power-up effects (8 particles) | ✅ Perfect | ✅ Overkill | ✅ Good |
| Trail sparkles (20-40 particles) | ✅ Good | ✅ Excellent | ✅ Good |
| Bundle size | ✅ ~5KB | ❌ +500KB | ⚠️ +500KB |
| Implementation time | ⚠️ 2-3 hours | ⚠️ 4-5 hours | ❌ 6-8 hours |
| Maintenance | ✅ Simple | ⚠️ Moderate | ⚠️ Complex |
| WebGL compatibility | ✅ Universal | ⚠️ WebGL2 only | ⚠️ WebGL2 only |

**Final Verdict**: 
- **MVP+**: Custom sprite-based particles (simple, effective, universal)
- **Post-MVP Polish**: Consider Hanabi for ambient/atmospheric effects only

---

## 4. Final Implementation Plan

### Phase 1: Critical Fix - HUD Visibility (Immediate)

**Priority**: 🔴 **CRITICAL - BLOCKS TESTING**

**Tasks**:
1. ✅ Download FiraSans-Bold.ttf (or Orbitron for sci-fi theme)
2. ✅ Create `assets/fonts/` directory
3. ✅ Place font file in directory
4. ✅ Modify `setup()` to load font via `AssetServer`
5. ✅ Update all `TextStyle` instances with font handle
6. ✅ Test on native build
7. ✅ Test on WASM build
8. ✅ Verify all HUD elements visible

**Estimated Time**: 30 minutes  
**Bundle Impact**: +75KB  
**Testing Checklist**:
- [ ] Score displays and updates
- [ ] Health shows correct value
- [ ] Combo multiplier visible
- [ ] Buffs (damage/shield) display
- [ ] Status message readable
- [ ] Text crisp and clear (no blurriness)

---

### Phase 2: Physics Polish (High Priority)

**Priority**: 🟡 **HIGH - IMPROVES FEEL**

**Implementation Order**:

**2.1 - Player Momentum** (1 hour):
- Add `Velocity` component to player
- Implement lerp-based acceleration
- Tune acceleration value (0.15 feels good)
- Test: Player should feel "weightier" but still responsive

**2.2 - Enemy Steering Behaviors** (1.5 hours):
- Add `Velocity` component to enemies
- Implement `steering_seek()` function
- Replace direct movement with velocity-based
- Add max turn rate constraint
- Test: Enemies should curve smoothly toward player

**2.3 - Spatial Hash Collision Optimization** (2 hours):
- Implement `SpatialHash` resource
- Update collision systems to use spatial queries
- Profile performance improvement
- Test: Should handle 50+ enemies without frame drops

**Estimated Time**: 4.5 hours  
**Bundle Impact**: 0KB  
**Performance Gain**: ~3-5x collision detection speedup  

**Testing Checklist**:
- [ ] Player movement feels smooth and weighted
- [ ] Enemies arc naturally (no instant turns)
- [ ] No noticeable performance drop with 50+ enemies
- [ ] Trail collision still works correctly
- [ ] Player collision still works correctly

---

### Phase 3: Particle System (Medium Priority)

**Priority**: 🟢 **MEDIUM - VISUAL JUICE**

**Implementation Order**:

**3.1 - Core Particle Component** (30 min):
- Create `Particle` component with lifecycle
- Implement `update_particles()` system
- Test with simple spawns

**3.2 - Enemy Death Explosions** (45 min):
- Implement `spawn_death_explosion()` 
- Hook into enemy defeat
- Tune: count=12, duration=0.6s, radial pattern
- Test: Should feel satisfying and immediate

**3.3 - Power-Up Pickup Effects** (45 min):
- Implement `spawn_pickup_effect()`
- Color-match power-up type
- Ring expansion pattern
- Test: Should clearly indicate pickup success

**3.4 - Damage Flash** (30 min):
- Implement screen-edge flash effect
- Brief white flash on player sprite
- Test: Should be noticeable but not annoying

**3.5 - Trail Sparkles** (optional, 30 min):
- Subtle occasional sparkles along trail
- Very low opacity (0.3-0.4)
- Test: Should enhance without distracting

**Estimated Time**: 3 hours (2.5 hours without trail sparkles)  
**Bundle Impact**: ~5KB  
**Visual Impact**: ⭐⭐⭐⭐⭐ High satisfaction

**Testing Checklist**:
- [ ] Death explosions are satisfying
- [ ] Pickup effects clearly visible
- [ ] Damage flash noticeable but not jarring
- [ ] No frame rate impact with 50+ active particles
- [ ] Particles despawn properly (no memory leak)

---

### Phase 4: Advanced Features (Post-MVP)

**Priority**: 🔵 **LOW - NICE TO HAVE**

**Potential Additions**:

**4.1 - Camera Shake**:
```rust
#[derive(Component)]
struct CameraShake {
    trauma: f32, // 0.0 to 1.0
}

fn update_camera_shake(
    time: Res<Time>,
    mut camera: Query<(&mut Transform, &mut CameraShake), With<MainCamera>>,
) {
    let (mut transform, mut shake) = camera.single_mut();
    
    if shake.trauma > 0.0 {
        let trauma_sq = shake.trauma * shake.trauma;
        let offset = Vec3::new(
            (random::<f32>() - 0.5) * trauma_sq * 10.0,
            (random::<f32>() - 0.5) * trauma_sq * 10.0,
            0.0,
        );
        transform.translation = offset;
        
        shake.trauma -= time.delta_seconds() * 2.0;
        shake.trauma = shake.trauma.max(0.0);
    } else {
        transform.translation = Vec3::ZERO;
    }
}

// Trigger shake on damage:
shake.trauma = 0.5; // Moderate shake
```

**4.2 - Combo Milestone Effects**:
- Text popup with scale animation
- Brief slow-motion (TimeScale = 0.7 for 0.2s)
- Radial particle burst
- Screen flash in combo color

**4.3 - Additional Enemy Types**:
- **Floater**: Slow, tries to stay at distance, shoots occasionally
- **Dasher**: Stops, charges in bursts
- **Orbiter**: Circles player while slowly closing in
- **Splitter**: Splits into 2 smaller enemies on death

**4.4 - Advanced Particles with Hanabi** (if bundle size allows):
- Background atmospheric dust
- Arena edge glow
- Power-up aura effects

---

## Implementation Timeline

### Sprint 1: MVP+ Core (1 day)
- **Morning (4 hours)**: HUD fix + testing
- **Afternoon (4 hours)**: Physics momentum + steering

**Deliverable**: Playable with visible HUD and improved feel

### Sprint 2: Polish (1 day)
- **Morning (3 hours)**: Particle system foundation + death explosions
- **Mid-day (2 hours)**: Power-up effects + damage flash  
- **Afternoon (3 hours)**: Spatial hash optimization + testing

**Deliverable**: Polished MVP ready for external testing

### Sprint 3: Advanced (Optional, 1-2 days)
- Camera shake
- Combo effects
- New enemy types
- Consider Hanabi integration

---

## Technical Specifications Summary

### Bundle Size Impact
| Feature | WASM Size | Native Size |
|---------|-----------|-------------|
| **Baseline (Current)** | ~2.5MB | ~8MB |
| + Font (FiraSans) | +75KB | +75KB |
| + Custom Physics | +5KB | +5KB |
| + Custom Particles | +5KB | +5KB |
| **Total MVP+** | **~2.6MB** | **~8.1MB** |
| | | |
| + bevy_rapier2d (optional) | +800KB | +1.2MB |
| + bevy_hanabi (optional) | +500KB | +600KB |
| **Full Featured** | **~3.9MB** | **~9.9MB** |

### Performance Targets
| Scenario | Target FPS | Notes |
|----------|------------|-------|
| 10 enemies, 50 trail segments | 60 FPS | Easy |
| 25 enemies, 90 trail segments | 60 FPS | Current target |
| 50 enemies, 120 trail segments | 60 FPS | With spatial hash |
| 100 enemies, 150 trail segments | 45-55 FPS | Stress test |

### Feature Compatibility Matrix
| Feature | Native | WASM/WebGL2 | WASM/WebGL1 |
|---------|--------|-------------|-------------|
| Custom Font | ✅ | ✅ | ✅ |
| Custom Physics | ✅ | ✅ | ✅ |
| Custom Particles | ✅ | ✅ | ✅ |
| bevy_rapier2d | ✅ | ✅ | ⚠️ Degraded |
| bevy_hanabi | ✅ | ✅ | ❌ No compute |

---

## Risk Assessment

### High Risk (Must Address)
- ❌ **HUD not visible** - Blocks all testing
  - **Mitigation**: Font integration (Phase 1)
  
### Medium Risk
- ⚠️ **Performance with many entities** - May drop below 60 FPS
  - **Mitigation**: Spatial hash optimization (Phase 2.3)
- ⚠️ **WASM bundle size growth** - May affect load times
  - **Mitigation**: Use custom implementations, avoid heavy crates

### Low Risk
- ✅ **Physics feel subjective** - Some may prefer current
  - **Mitigation**: Make momentum strength configurable const
- ✅ **Particle aesthetics** - Colors/timing may need tuning
  - **Mitigation**: Quick to iterate, easy to adjust

---

## Success Metrics

### Phase 1 Success (HUD):
- ✅ All text visible on first launch
- ✅ Updates reflect game state accurately
- ✅ Readable at various screen sizes
- ✅ Works on both native and web

### Phase 2 Success (Physics):
- ✅ Player movement feels "juicy" and responsive
- ✅ Enemies move naturally (no robotic straight lines)
- ✅ 60 FPS with 50 enemies + 120 trail segments
- ✅ No collision detection bugs

### Phase 3 Success (Particles):
- ✅ Enemy deaths feel satisfying
- ✅ Power-up pickups clearly visible
- ✅ Damage feedback immediate and clear
- ✅ No performance impact (60 FPS maintained)
- ✅ 100+ particles active without issues

---

## Conclusion & Recommendations

### Immediate Actions (Today):
1. **FIX HUD** - Add font asset (critical)
2. **Test thoroughly** - Verify all text visible
3. **Begin Phase 2** - Add momentum to player

### This Week:
4. Complete physics improvements (momentum + steering)
5. Implement particle system foundation
6. Add enemy death explosions and power-up effects

### Next Week:
7. Optimize with spatial hash
8. Add remaining particle effects
9. Conduct performance testing
10. Gather user feedback

### Future Considerations:
- Monitor bundle size if adding external physics/particles
- Consider Avian physics if we need advanced features
- Evaluate Hanabi for atmospheric effects only
- Plan enemy variety based on gameplay feedback

---

**Document Version**: 1.0  
**Last Updated**: October 17, 2025  
**Next Review**: After Phase 1 completion

