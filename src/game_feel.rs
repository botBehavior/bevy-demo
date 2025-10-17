// Game Feel & Juice Systems
// Implements screen shake, knockback, and other satisfying effects

use bevy::prelude::*;

// Constants for game feel tuning
pub const SCREEN_SHAKE_DECAY: f32 = 3.0; // How fast shake fades
pub const KNOCKBACK_STRENGTH: f32 = 300.0; // How far enemies get knocked back
pub const KNOCKBACK_DECAY: f32 = 8.0; // How fast knockback fades
pub const HIT_FREEZE_DURATION: f32 = 0.05; // Brief pause on hit for impact
pub const PLAYER_KNOCKBACK: f32 = 150.0; // Player knockback when hit

#[derive(Component)]
pub struct ScreenShake {
    pub trauma: f32, // 0.0 to 1.0, squared for intensity
}

impl Default for ScreenShake {
    fn default() -> Self {
        Self { trauma: 0.0 }
    }
}

impl ScreenShake {
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).min(1.0);
    }
}

#[derive(Component)]
pub struct Knockback {
    pub velocity: Vec2,
}

impl Default for Knockback {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
        }
    }
}

#[derive(Resource)]
pub struct HitFreeze {
    pub timer: Timer,
}

impl Default for HitFreeze {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}

impl HitFreeze {
    pub fn trigger(&mut self) {
        self.timer = Timer::from_seconds(HIT_FREEZE_DURATION, TimerMode::Once);
    }

    pub fn is_active(&self) -> bool {
        !self.timer.finished()
    }

    pub fn update(&mut self, delta: f32) {
        self.timer.tick(std::time::Duration::from_secs_f32(delta));
    }
}

// Screen shake system
pub fn update_screen_shake(
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut ScreenShake)>,
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
            
            // Apply shake offset
            transform.translation.x = offset.x;
            transform.translation.y = offset.y;
        } else if transform.translation.x != 0.0 || transform.translation.y != 0.0 {
            // Reset camera position
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
        }
    }
}

// Knockback system
pub fn apply_knockback(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Knockback)>,
) {
    for (mut transform, mut knockback) in &mut query {
        if knockback.velocity.length_squared() > 1.0 {
            transform.translation += knockback.velocity.extend(0.0) * time.delta_seconds();
            knockback.velocity *= 1.0 - (KNOCKBACK_DECAY * time.delta_seconds());
        } else {
            knockback.velocity = Vec2::ZERO;
        }
    }
}

