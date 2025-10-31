use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub weapon: WeaponType,
    pub wave_cooldown: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            weapon: WeaponType::Trail,
            wave_cooldown: 0.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WeaponType {
    Trail,
    Wave,
}

#[derive(Component)]
pub struct PlayerVelocity {
    pub current: Vec2,
}

impl Default for PlayerVelocity {
    fn default() -> Self {
        Self {
            current: Vec2::ZERO,
        }
    }
}

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
}

#[derive(Component)]
pub struct EnemyHealth {
    pub current: f32,
}

#[derive(Component)]
pub struct EnemyVelocity {
    pub current: Vec2,
}

impl Default for EnemyVelocity {
    fn default() -> Self {
        Self {
            current: Vec2::ZERO,
        }
    }
}

#[derive(Component)]
pub struct TrailSegment {
    pub remaining: f32,
    pub damage: f32,
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

#[derive(Component)]
pub struct WaveProjectile {
    pub velocity: Vec2,
    pub age: f32,
    pub lifetime: f32,
    pub damage: u32,
}

#[derive(Component)]
pub struct Particle {
    pub velocity: Vec2,
    pub lifetime: f32,
    pub age: f32,
}

#[derive(Component)]
pub struct PowerUp {
    pub kind: PowerUpKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PowerUpKind {
    Currency,
    Health,
    Shield,
    Accuracy,
    WaveBlast,
}

#[derive(Component)]
pub struct PowerUpLifetime {
    pub timer: Timer,
}

#[derive(Component)]
pub struct ScreenShake {
    pub trauma: f32,
    pub decay: f32,
}

impl Default for ScreenShake {
    fn default() -> Self {
        Self {
            trauma: 0.0,
            decay: 3.0,
        }
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct HudScore;

#[derive(Component)]
pub struct HudHealth;

#[derive(Component)]
pub struct HudCombo;

#[derive(Component)]
pub struct HudStatus;

#[derive(Component)]
pub struct HudBuffs;

#[derive(Component)]
pub struct HudHealthBar;

#[derive(Component)]
pub struct ShopRoot;

#[derive(Component)]
pub struct ShopButton;

#[derive(Component)]
pub struct ShopModal;

#[derive(Component)]
pub struct ShopCard {
    pub index: usize,
}

#[derive(Component)]
pub struct ShopPurchaseButton {
    pub index: usize,
}

#[derive(Component)]
pub struct ShopCostText(pub usize);

#[derive(Component)]
pub struct ShopDescriptionText(pub usize);

#[derive(Component)]
pub struct ShopLevelText(pub usize);
