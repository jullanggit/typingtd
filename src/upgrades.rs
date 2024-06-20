use std::fmt::Display;

use bevy::{prelude::*, utils::HashMap};
use strum::EnumIter;

pub struct UpgradePlugin;
impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ArrowTowerUpgrades>();
    }
}

#[derive(Debug, Clone, Reflect, PartialEq, Eq, Hash, EnumIter)]
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
    pub upgrades: HashMap<ArrowTowerUpgradeType, u8>,
}

pub fn upgrade_tower(
    In((tower, upgrade)): In<(Entity, ArrowTowerUpgradeType)>,
    mut upgrades: Query<&mut ArrowTowerUpgrades>,
) {
    let mut tower_upgrades = upgrades
        .get_mut(tower)
        .expect("Provided Entity should exist / have the TowerUpgrades component");

    // Check if the tower already has the given upgrade
    if let Some(level) = tower_upgrades.upgrades.get_mut(&upgrade) {
        // If possible, upgrade the level of the upgrade
        if *level < upgrade.max_level() {
            *level += 1;
        }
    // Otherwise add the upgrade
    } else {
        tower_upgrades.upgrades.insert(upgrade, 0);
    }
}
