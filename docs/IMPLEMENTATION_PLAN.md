# Threadweaver Improvements - Implementation Plan
## Quick Reference Guide

**Based on**: [Research Findings](research-findings-improvements.md)  
**Date**: October 17, 2025

---

## 🎯 Executive Summary

| Priority | Feature | Time | Impact | Bundle Cost |
|----------|---------|------|--------|-------------|
| 🔴 CRITICAL | Fix HUD visibility | 30 min | ⭐⭐⭐⭐⭐ | +75KB |
| 🟡 HIGH | Player momentum | 1 hour | ⭐⭐⭐⭐ | 0KB |
| 🟡 HIGH | Enemy steering | 1.5 hours | ⭐⭐⭐⭐ | 0KB |
| 🟢 MEDIUM | Death explosions | 45 min | ⭐⭐⭐⭐ | ~2KB |
| 🟢 MEDIUM | Pickup effects | 45 min | ⭐⭐⭐ | ~2KB |
| 🟢 MEDIUM | Damage flash | 30 min | ⭐⭐⭐ | ~1KB |
| 🔵 LOW | Spatial hash | 2 hours | ⭐⭐⭐ | 0KB |
| 🔵 LOW | Trail sparkles | 30 min | ⭐⭐ | ~1KB |

**Total MVP+ Time**: ~7 hours  
**Total Bundle Cost**: ~81KB  
**Expected Result**: Polished, juicy gameplay experience

---

## 🔴 Phase 1: HUD Fix (CRITICAL)

### Problem
Text not rendering because Bevy 0.14 has no default font.

### Solution
Load explicit font via AssetServer.

### Step-by-Step

1. **Get a font file** (choose one):
   - FiraSans-Bold.ttf - [Download](https://fonts.google.com/specimen/Fira+Sans)
   - Orbitron-Bold.ttf - [Download](https://fonts.google.com/specimen/Orbitron) (sci-fi theme)
   - Roboto-Bold.ttf - [Download](https://fonts.google.com/specimen/Roboto)

2. **Create directory**:
   ```bash
   mkdir -p assets/fonts
   ```

3. **Place font**:
   ```bash
   cp ~/Downloads/FiraSans-Bold.ttf assets/fonts/
   ```

4. **Modify setup() function**:
   ```rust
   fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
       commands.spawn((Camera2dBundle::default(), MainCamera));
       
       // Load font
       let font = asset_server.load("fonts/FiraSans-Bold.ttf");
       
       // ... (rest of setup)
   ```

5. **Update all TextStyle instances**:
   ```rust
   TextStyle {
       font: font.clone(),  // Add this line
       font_size: 22.0,
       color: Color::WHITE,
   }
   ```
   
   Do this for ALL 5 HUD text elements:
   - HudScore
   - HudHealth
   - HudCombo
   - HudBuffs
   - HudStatus

6. **Test**:
   ```bash
   cargo run  # Native
   trunk serve  # Web
   ```

**Success Criteria**: All text visible and readable

---

## 🟡 Phase 2A: Player Momentum

### Goal
Make player movement feel more "weighty" and satisfying.

### Implementation

1. **Add Velocity component**:
   ```rust
   #[derive(Component)]
   struct Velocity {
       linear: Vec2,
   }
   ```

2. **Add to player spawn**:
   ```rust
   commands.spawn((
       SpriteBundle { /* ... */ },
       Player,
       Velocity { linear: Vec2::ZERO },  // NEW
   ));
   ```

3. **Update move_player() system**:
   ```rust
   fn move_player(
       time: Res<Time>,
       run_state: Res<RunState>,
       target: Res<PointerTarget>,
       mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
   ) {
       if !run_state.is_running() {
           return;
       }

       let (mut transform, mut velocity) = query.single_mut();
       let current = transform.translation.truncate();
       let delta = target.position - current;
       let distance = delta.length();

       if distance > f32::EPSILON {
           let desired = delta.normalize() * PLAYER_SPEED;
           velocity.linear = velocity.linear.lerp(desired, 0.15);
       } else {
           velocity.linear = velocity.linear.lerp(Vec2::ZERO, 0.3);
       }

       transform.translation += velocity.linear.extend(0.0) * time.delta_seconds();
       clamp_to_bounds(&mut transform.translation);
   }
   ```

4. **Add constant for tuning**:
   ```rust
   const PLAYER_ACCELERATION: f32 = 0.15;  // 0.1=sluggish, 0.3=snappy
   ```

5. **Test and tune**:
   - Try values 0.1 - 0.3
   - Should feel smooth but responsive

---

## 🟡 Phase 2B: Enemy Steering

### Goal
Make enemies move organically instead of robotic straight lines.

### Implementation

1. **Add Velocity to Enemy**:
   ```rust
   commands.spawn((
       SpriteBundle { /* ... */ },
       Enemy { speed: enemy_speed },
       EnemyHealth { current: ENEMY_BASE_HEALTH as f32 },
       Velocity { linear: Vec2::ZERO },  // NEW
   ));
   ```

2. **Create steering function**:
   ```rust
   fn steering_seek(
       current_pos: Vec2,
       target_pos: Vec2,
       current_velocity: Vec2,
       max_speed: f32,
   ) -> Vec2 {
       let desired = (target_pos - current_pos).normalize() * max_speed;
       let steering = desired - current_velocity;
       steering.clamp_length_max(max_speed * 0.15)
   }
   ```

3. **Update move_enemies() system**:
   ```rust
   fn move_enemies(
       time: Res<Time>,
       run_state: Res<RunState>,
       player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
       mut enemies: Query<(&Enemy, &mut Transform, &mut Velocity), Without<Player>>,
   ) {
       if !run_state.is_running() {
           return;
       }

       let player_pos = player_query.single().translation.truncate();
       
       for (enemy, mut transform, mut velocity) in &mut enemies {
           let pos = transform.translation.truncate();
           let steering = steering_seek(pos, player_pos, velocity.linear, enemy.speed);
           
           velocity.linear += steering;
           velocity.linear = velocity.linear.clamp_length_max(enemy.speed);
           
           transform.translation += velocity.linear.extend(0.0) * time.delta_seconds();
       }
   }
   ```

4. **Test**:
   - Enemies should curve smoothly
   - No instant direction changes
   - Should still feel threatening

---

## 🟢 Phase 3A: Particle Foundation

### Setup

1. **Add Particle component**:
   ```rust
   #[derive(Component)]
   struct Particle {
       velocity: Vec2,
       lifetime: Timer,
       start_size: f32,
       end_size: f32,
       start_alpha: f32,
       end_alpha: f32,
   }

   impl Particle {
       fn new(velocity: Vec2, lifetime: f32) -> Self {
           Self {
               velocity,
               lifetime: Timer::from_seconds(lifetime, TimerMode::Once),
               start_size: 8.0,
               end_size: 2.0,
               start_alpha: 1.0,
               end_alpha: 0.0,
           }
       }
   }
   ```

2. **Add update system**:
   ```rust
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
           
           // Move
           transform.translation += particle.velocity.extend(0.0) * time.delta_seconds();
           
           // Drag
           particle.velocity *= 0.95;
           
           // Fade and shrink
           let t = particle.lifetime.fraction();
           let size = particle.start_size.lerp(particle.end_size, t);
           let alpha = particle.start_alpha.lerp(particle.end_alpha, t);
           
           sprite.custom_size = Some(Vec2::splat(size));
           sprite.color.set_alpha(alpha);
       }
   }
   ```

3. **Register system**:
   ```rust
   .add_systems(Update, (
       // ... existing systems ...
       update_particles,
   ))
   ```

---

## 🟢 Phase 3B: Death Explosions

### Implementation

1. **Create spawn function**:
   ```rust
   fn spawn_death_explosion(commands: &mut Commands, position: Vec2) {
       use std::f32::consts::TAU;
       const PARTICLE_COUNT: usize = 12;
       
       for i in 0..PARTICLE_COUNT {
           let angle = (i as f32 / PARTICLE_COUNT as f32) * TAU;
           let speed = 150.0 + rand::random::<f32>() * 100.0;
           let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;
           
           commands.spawn((
               SpriteBundle {
                   sprite: Sprite {
                       color: Color::srgba(1.0, 0.5, 0.5, 0.9),
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
   ```

2. **Hook into enemy death** (in `handle_trail_collisions`):
   ```rust
   if !defeated.is_empty() {
       for (entity, position) in defeated {
           score.current += combo.register_kill();
           spawn_death_explosion(&mut commands, position);  // NEW
           maybe_spawn_power_up(&mut commands, rng, position);
           commands.entity(entity).despawn_recursive();
       }
   }
   ```

3. **Test**: Should see radial particle burst on kill

---

## 🟢 Phase 3C: Pickup Effects

### Implementation

```rust
fn spawn_pickup_effect(commands: &mut Commands, position: Vec2, color: Color) {
    use std::f32::consts::TAU;
    
    for i in 0..8 {
        let angle = (i as f32 / 8.0) * TAU;
        let offset = Vec2::new(angle.cos(), angle.sin()) * 20.0;
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(6.0)),
                    ..default()
                },
                transform: Transform::from_translation((position + offset).extend(0.3)),
                ..default()
            },
            Particle {
                velocity: offset * 3.0,
                lifetime: Timer::from_seconds(0.4, TimerMode::Once),
                start_size: 6.0,
                end_size: 0.0,
                start_alpha: 1.0,
                end_alpha: 0.0,
            },
        ));
    }
}
```

Hook into `handle_power_up_pickups`:
```rust
match power_up.kind {
    PowerUpKind::Heart => {
        player_health.heal(1);
        spawn_pickup_effect(&mut commands, transform.translation.truncate(), 
                           Color::srgba(1.0, 0.3, 0.4, 1.0));
    }
    PowerUpKind::Shield => {
        shield.activate();
        spawn_pickup_effect(&mut commands, transform.translation.truncate(),
                           Color::srgba(0.4, 0.8, 1.0, 1.0));
    }
    PowerUpKind::Damage => {
        combat.add_bonus();
        spawn_pickup_effect(&mut commands, transform.translation.truncate(),
                           Color::srgba(1.0, 0.7, 0.2, 1.0));
    }
}
```

---

## 🟢 Phase 3D: Damage Flash

### Implementation

```rust
#[derive(Component)]
struct DamageFlash {
    timer: Timer,
}

fn spawn_damage_flash(commands: &mut Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 0.0, 0.0, 0.3),
                custom_size: Some(ARENA_BOUNDS * 1.5),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.9)),
            ..default()
        },
        DamageFlash {
            timer: Timer::from_seconds(0.15, TimerMode::Once),
        },
    ));
}

fn update_damage_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut flashes: Query<(Entity, &mut DamageFlash, &mut Sprite)>,
) {
    for (entity, mut flash, mut sprite) in &mut flashes {
        flash.timer.tick(time.delta());
        
        if flash.timer.finished() {
            commands.entity(entity).despawn();
        } else {
            let alpha = 0.3 * (1.0 - flash.timer.fraction());
            sprite.color.set_alpha(alpha);
        }
    }
}
```

Hook into `handle_player_collisions`:
```rust
if !shield.is_active() {
    health.apply_damage(PLAYER_COLLISION_DAMAGE);
    spawn_damage_flash(&mut commands);  // NEW
    
    if health.current == 0 {
        // ...
    }
}
```

---

## 🔵 Phase 4: Spatial Hash (Optional Optimization)

Only implement if experiencing performance issues with 50+ enemies.

### When to Use
- FPS drops below 55 with many enemies
- Profiler shows collision detection as bottleneck

### Implementation

```rust
const SPATIAL_CELL_SIZE: f32 = 100.0;

#[derive(Resource, Default)]
struct SpatialHash {
    cells: HashMap<(i32, i32), Vec<Entity>>,
}

impl SpatialHash {
    fn clear(&mut self) {
        self.cells.clear();
    }
    
    fn insert(&mut self, entity: Entity, pos: Vec2) {
        let cell = self.pos_to_cell(pos);
        self.cells.entry(cell).or_default().push(entity);
    }
    
    fn pos_to_cell(&self, pos: Vec2) -> (i32, i32) {
        (
            (pos.x / SPATIAL_CELL_SIZE).floor() as i32,
            (pos.y / SPATIAL_CELL_SIZE).floor() as i32,
        )
    }
    
    fn get_nearby(&self, pos: Vec2, radius: f32) -> Vec<Entity> {
        let center = self.pos_to_cell(pos);
        let mut nearby = Vec::new();
        
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

// Use in collision system:
fn handle_trail_collisions(
    // ... existing params ...
    mut spatial_hash: ResMut<SpatialHash>,
) {
    // Clear and rebuild hash each frame
    spatial_hash.clear();
    for (entity, transform) in &trail {
        spatial_hash.insert(entity, transform.translation.truncate());
    }
    
    for (enemy_entity, enemy_transform, _enemy, mut health) in &mut enemies {
        let enemy_pos = enemy_transform.translation.truncate();
        
        // Only check nearby trail segments
        for trail_entity in spatial_hash.get_nearby(enemy_pos, TRAIL_HIT_RADIUS) {
            // ... collision check ...
        }
    }
}
```

---

## 📋 Testing Checklist

### After Phase 1 (HUD):
- [ ] Score displays correctly
- [ ] Health shows X/5 format
- [ ] Combo updates in real-time
- [ ] Buff text readable
- [ ] Status messages clear
- [ ] Works on native
- [ ] Works on web

### After Phase 2 (Physics):
- [ ] Player movement feels smooth
- [ ] No "instant snap" to cursor
- [ ] Enemies curve naturally
- [ ] No robotic straight-line movement
- [ ] Enemies still catch player eventually
- [ ] 60 FPS with 25 enemies

### After Phase 3 (Particles):
- [ ] Death explosions look good
- [ ] 12 particles radiate outward
- [ ] Pickup ring expands nicely
- [ ] Color matches power-up type
- [ ] Damage flash is noticeable
- [ ] Red flash fades quickly
- [ ] No FPS drops with particles

### Performance Targets:
- [ ] 60 FPS with 10 enemies
- [ ] 60 FPS with 25 enemies
- [ ] 55+ FPS with 50 enemies
- [ ] 50+ FPS with 100 particles

---

## 🐛 Common Issues & Solutions

### HUD still not visible
- **Check**: Font file exists at `assets/fonts/FiraSans-Bold.ttf`
- **Check**: All TextStyle have `font: font.clone()`
- **Check**: `setup()` accepts `AssetServer`
- **Try**: Different font file
- **Try**: Absolute path for testing

### Player too sluggish
- **Increase** `PLAYER_ACCELERATION` (try 0.2 or 0.25)
- **Reduce** deceleration lerp factor

### Player too snappy (no momentum feel)
- **Decrease** `PLAYER_ACCELERATION` (try 0.1 or 0.12)
- **Increase** deceleration lerp factor

### Enemies miss player
- **Reduce** steering clamp (try 0.1 instead of 0.15)
- **Increase** enemy max speed

### Particles lag
- **Reduce** particle count (8 instead of 12 for deaths)
- **Shorten** lifetime (0.4s instead of 0.6s)
- **Check**: Particles despawning properly

### Frame drops with many entities
- **Implement** spatial hash (Phase 4)
- **Profile** to find bottleneck
- **Consider** reducing trail lifetime
- **Consider** reducing spawn rate

---

## 🎯 Success Metrics

**MVP+ is successful when**:
1. ✅ HUD visible and updating correctly
2. ✅ Player movement feels "juicy" (momentum)
3. ✅ Enemies move organically (steering)
4. ✅ Deaths are satisfying (particles)
5. ✅ Pickups feel rewarding (ring effect)
6. ✅ Damage clearly communicated (flash)
7. ✅ 60 FPS with 25+ enemies
8. ✅ Testers describe it as "polished"

---

## 📞 Quick Reference

### File Locations
- Main code: `src/main.rs`
- Font assets: `assets/fonts/`
- Docs: `docs/`

### Build Commands
```bash
# Native
cargo run
cargo build --release

# Web
trunk serve
trunk build --release

# Checks
cargo check
cargo clippy
cargo fmt
```

### Key Constants
```rust
PLAYER_SPEED: f32 = 900.0
PLAYER_ACCELERATION: f32 = 0.15
ENEMY_TURN_RATE: f32 = 0.15
PARTICLE_LIFETIME: f32 = 0.6
SPATIAL_CELL_SIZE: f32 = 100.0
```

---

**Ready to implement?** Start with Phase 1 (HUD fix) and work through sequentially.

