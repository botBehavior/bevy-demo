use crate::resources::PurchasedUpgrades;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UpgradeCategory {
    Combat,
    Survival,
    Utility,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum PlayerColor {
    Default,
    Red,
    Blue,
    Purple,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UpgradeType {
    MovementSpeed,
    MaxHealth,
    TrailDamage,
    ShieldDuration,
}

impl UpgradeType {
    pub fn display_name(&self) -> &'static str {
        match self {
            UpgradeType::MovementSpeed => "Movement Speed",
            UpgradeType::MaxHealth => "Maximum Health",
            UpgradeType::TrailDamage => "Trail Damage",
            UpgradeType::ShieldDuration => "Shield Duration",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            UpgradeType::MovementSpeed => "??",
            UpgradeType::MaxHealth => "??",
            UpgradeType::TrailDamage => "??",
            UpgradeType::ShieldDuration => "???",
        }
    }

    pub fn category(&self) -> UpgradeCategory {
        match self {
            UpgradeType::MovementSpeed => UpgradeCategory::Utility,
            UpgradeType::MaxHealth => UpgradeCategory::Survival,
            UpgradeType::TrailDamage => UpgradeCategory::Combat,
            UpgradeType::ShieldDuration => UpgradeCategory::Survival,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ShopItem {
    pub upgrade: UpgradeType,
    pub description: &'static str,
    pub base_cost: u32,
    pub max_level: u32,
}

impl ShopItem {
    pub const fn new(
        upgrade: UpgradeType,
        description: &'static str,
        base_cost: u32,
        max_level: u32,
    ) -> Self {
        Self {
            upgrade,
            description,
            base_cost,
            max_level,
        }
    }

    pub fn cost_for_level(&self, level: u32) -> u32 {
        let next = level + 1;
        match self.upgrade {
            UpgradeType::MovementSpeed => self.base_cost * next * next,
            UpgradeType::TrailDamage => self.base_cost * next,
            UpgradeType::ShieldDuration => self.base_cost * (next + 1),
            UpgradeType::MaxHealth => self.base_cost * next.pow(2),
        }
    }

    pub fn is_maxed(&self, upgrades: &PurchasedUpgrades) -> bool {
        let level = match self.upgrade {
            UpgradeType::MovementSpeed => upgrades.movement_speed_level,
            UpgradeType::MaxHealth => upgrades.max_health_level,
            UpgradeType::TrailDamage => upgrades.trail_damage_level,
            UpgradeType::ShieldDuration => upgrades.shield_level,
        };
        level >= self.max_level
    }
}

pub const SHOP_ITEMS: &[ShopItem] = &[
    ShopItem::new(
        UpgradeType::MovementSpeed,
        "Move quicker across the weave.",
        12,
        4,
    ),
    ShopItem::new(
        UpgradeType::MaxHealth,
        "Increase maximum resolve by one.",
        15,
        4,
    ),
    ShopItem::new(
        UpgradeType::TrailDamage,
        "Empower the light trail for more damage.",
        18,
        3,
    ),
    ShopItem::new(
        UpgradeType::ShieldDuration,
        "Extend the protective weave shield.",
        20,
        3,
    ),
];
