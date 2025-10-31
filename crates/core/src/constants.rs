use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 950.0;
pub const PLAYER_RADIUS: f32 = 14.0;
pub const PLAYER_MAX_HEALTH: u32 = 4;
pub const PLAYER_COLLISION_DAMAGE: u32 = 1;
pub const TRAIL_LIFETIME: f32 = 2.6;
pub const TRAIL_SPAWN_INTERVAL: f32 = 0.028;
pub const TRAIL_HIT_RADIUS: f32 = 16.0;
pub const ENEMY_BASE_SPEED: f32 = 180.0;
pub const ENEMY_SPEED_INCREMENT: f32 = 8.0;
pub const ENEMY_SPAWN_INTERVAL_START: f32 = 2.0;
pub const ENEMY_SPAWN_ACCELERATION: f32 = 0.92;
pub const ENEMY_SIZE: Vec2 = Vec2::new(36.0, 36.0);
pub const COMBO_WINDOW: f32 = 1.0;
pub const COMBO_MULTIPLIER_STEP: f32 = 0.5;
pub const BASE_SCORE: u32 = 10;
pub const ARENA_BOUNDS: Vec2 = Vec2::new(1024.0, 768.0);
pub const ENEMY_BASE_HEALTH: u32 = 3;
pub const TRAIL_BASE_DAMAGE: u32 = 3;
pub const SHIELD_DURATION: f32 = 4.0;

pub const POWER_UP_LIFETIME: f32 = 12.0;
pub const POWER_UP_DROP_CHANCE: f32 = 0.15;
pub const POWER_UP_HEART_WEIGHT: f32 = 0.35;
pub const POWER_UP_SHIELD_WEIGHT: f32 = 0.25;
pub const POWER_UP_CURRENCY_WEIGHT: f32 = 0.15;
pub const POWER_UP_ACCURACY_WEIGHT: f32 = 0.15;
pub const POWER_UP_WAVEBLAST_WEIGHT: f32 = 0.10;

pub const SCREEN_SHAKE_DECAY: f32 = 3.0;
pub const ENEMY_KNOCKBACK: f32 = 250.0;
pub const PLAYER_KNOCKBACK_STRENGTH: f32 = 200.0;
pub const HIT_FREEZE_DURATION: f32 = 0.04;
pub const PLAYER_ACCELERATION: f32 = 0.12;
pub const PLAYER_DECELERATION: f32 = 0.25;
pub const ENEMY_TURN_SPEED: f32 = 0.18;

pub const CAMERA_SMOOTHING: f32 = 0.30;
pub const ARENA_SIZE: f32 = 5000.0;
pub const ENEMY_SPAWN_DISTANCE: f32 = 600.0;

pub const WAVE_COOLDOWN: f32 = 0.35;
pub const WAVE_PROJECTILE_COUNT: u32 = 5;
pub const WAVE_SPREAD_ANGLE: f32 = 0.4;
pub const WAVE_SPEED: f32 = 800.0;
pub const WAVE_LIFETIME: f32 = 1.5;
pub const WAVE_DAMAGE: u32 = 2;

pub const PLAYER_START_HEALTH: u32 = 4;
pub const PLAYER_BOOSTED_SPEED: f32 = 950.0;
pub const TRAIL_START_DAMAGE: u32 = 3;
pub const ENEMY_START_SPEED: f32 = 180.0;
pub const COMBO_TIGHTER_WINDOW: f32 = 1.0;
pub const SHIELD_TACTICAL_DURATION: f32 = 4.0;
pub const POWER_UP_RARE_CHANCE: f32 = 0.15;

pub const TOUCH_DRAG_DEADZONE: f32 = 12.0;
