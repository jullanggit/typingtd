use std::fmt::Display;

use bevy::prelude::*;
use strum::EnumIter;

pub struct UpgradePlugin;
impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ArrowTowerUpgrades>();
    }
}

#[derive(Debug, Clone, Reflect, PartialEq, Eq, EnumIter)]
pub enum ArrowTowerUpgradeType {
    Piercing,
    Multishot,
    Tracking,
}
impl ArrowTowerUpgradeType {
    const fn max_level(&self) -> u8 {
        match self {
            Self::Piercing => u8::MAX,
            Self::Multishot => 11,
            Self::Tracking => 5,
        }
    }
}
#[derive(Debug, Clone, Reflect, PartialEq, Eq)]
pub struct ArrowTowerUpgrade {
    pub upgrade_type: ArrowTowerUpgradeType,
    pub level: u8,
}
impl ArrowTowerUpgrade {
    pub const fn new(upgrade_type: ArrowTowerUpgradeType, level: u8) -> Self {
        Self {
            upgrade_type,
            level,
        }
    }
}

impl Display for ArrowTowerUpgradeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Piercing => "Piercing",
                Self::Multishot => "Multishot",
                Self::Tracking => "Tracking",
            }
        )
    }
}

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
#[repr(transparent)]
pub struct ArrowTowerUpgrades {
    pub upgrades: Vec<ArrowTowerUpgrade>,
}

pub fn upgrade_tower(
    In((tower, upgrade)): In<(Entity, ArrowTowerUpgradeType)>,
    mut upgrades: Query<&mut ArrowTowerUpgrades>,
) {
    let mut tower_upgrades = upgrades
        .get_mut(tower)
        .expect("Provided Entity should exist / have the TowerUpgrades component");

    // Check if the tower already has the given upgrade
    if let Some(present_upgrade) = tower_upgrades
        .upgrades
        .iter_mut()
        .find(|present_upgrade| present_upgrade.upgrade_type == upgrade)
    {
        // If possible, upgrade the level of the upgrade
        if present_upgrade.level < present_upgrade.upgrade_type.max_level() {
            present_upgrade.level += 1;
        }
    // Otherwise add the upgrade
    } else {
        tower_upgrades
            .upgrades
            .push(ArrowTowerUpgrade::new(upgrade, 0));
    }
}
