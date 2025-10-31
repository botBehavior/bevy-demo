use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Resource, Default)]
pub struct RunState {
    active: bool,
    paused: bool,
}

impl RunState {
    pub fn new() -> Self {
        Self {
            active: true,
            paused: false,
        }
    }

    pub fn is_running(&self) -> bool {
        self.active && !self.paused
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        if self.active {
            self.paused = false;
        }
    }

    pub fn reset(&mut self) {
        self.active = true;
        self.paused = false;
    }

    pub fn end(&mut self) {
        self.active = false;
    }

    pub fn toggle_pause(&mut self) {
        if self.active {
            self.paused = !self.paused;
        }
    }
}

#[derive(Resource, Default)]
pub struct PointerTarget {
    pub position: Vec2,
}

#[derive(Resource, Default)]
pub struct PlayerHealth {
    pub current: u32,
    pub max: u32,
}

impl PlayerHealth {
    pub fn with_max(max: u32) -> Self {
        Self { current: max, max }
    }

    pub fn heal(&mut self, amount: u32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn damage(&mut self, amount: u32) {
        self.current = self.current.saturating_sub(amount);
    }

    pub fn reset(&mut self) {
        self.current = self.max;
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
}

#[derive(Resource)]
pub struct PlayerStats {
    pub base_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            base_speed: 950.0,
            acceleration: 0.18,
            deceleration: 0.32,
        }
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub current: u32,
    pub best: u32,
}

impl Score {
    pub fn add(&mut self, amount: u32) {
        self.current += amount;
        self.best = self.best.max(self.current);
    }

    pub fn reset_run(&mut self) {
        self.current = 0;
    }
}

#[derive(Resource, Default)]
pub struct Currency {
    pub balance: u32,
}

impl Currency {
    pub fn credit(&mut self, amount: u32) {
        self.balance += amount;
    }

    pub fn debit(&mut self, amount: u32) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct PurchasedUpgrades {
    pub movement_speed_level: u32,
    pub max_health_level: u32,
    pub trail_damage_level: u32,
    pub shield_level: u32,
}

impl Default for PurchasedUpgrades {
    fn default() -> Self {
        Self {
            movement_speed_level: 0,
            max_health_level: 0,
            trail_damage_level: 0,
            shield_level: 0,
        }
    }
}

impl PurchasedUpgrades {
    pub fn movement_speed_multiplier(&self) -> f32 {
        1.0 + self.movement_speed_level as f32 * 0.1
    }

    pub fn max_health_bonus(&self) -> u32 {
        self.max_health_level * 1
    }

    pub fn trail_damage_multiplier(&self) -> f32 {
        1.0 + self.trail_damage_level as f32 * 0.2
    }

    pub fn shield_duration_bonus(&self) -> f32 {
        self.shield_level as f32 * 0.75
    }
}

#[derive(Resource, Default)]
pub struct ShopState {
    pub is_open: bool,
    pub selected_index: usize,
}

impl ShopState {
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }
}

#[derive(Resource, Default)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl EnemySpawnTimer {
    pub fn new(interval: f32) -> Self {
        let mut timer = Timer::from_seconds(interval, TimerMode::Repeating);
        timer.tick(Duration::from_secs_f32(interval));
        Self { timer }
    }
}

#[derive(Resource, Default)]
pub struct TrailSpawnTimer {
    pub timer: Timer,
}

#[derive(Resource, Default)]
pub struct ShieldState {
    pub remaining: f32,
    pub duration: f32,
}

impl ShieldState {
    pub fn is_active(&self) -> bool {
        self.remaining > 0.0
    }
}

#[derive(Resource, Default)]
pub struct GameAssets {
    pub font_primary: Handle<Font>,
    pub font_numbers: Handle<Font>,
    pub player: Handle<Image>,
    pub enemy: Handle<Image>,
    pub background: Handle<Image>,
    pub trail_segment: Handle<Image>,
    pub wave_projectile: Handle<Image>,
    pub powerup_currency: Handle<Image>,
    pub powerup_health: Handle<Image>,
    pub powerup_shield: Handle<Image>,
    pub powerup_accuracy: Handle<Image>,
    pub powerup_waveblast: Handle<Image>,
}
