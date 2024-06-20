use std::fmt::Display;

use bevy::prelude::*;
use strum::{EnumDiscriminants, EnumIter};

pub struct UpgradePlugin;
impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ArrowTowerUpgrades>();
    }
}

#[derive(Debug, Clone, Reflect, PartialEq, EnumIter, EnumDiscriminants)]
pub enum ArrowTowerUpgrade {
    Piercing(f64),
    Multishot(f64),
    Tracking,
}
impl Display for ArrowTowerUpgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Piercing(_) => "Piercing",
                Self::Multishot(_) => "Multishot",
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
    In((tower, upgrade)): In<(Entity, ArrowTowerUpgrade)>,
    mut upgrades: Query<&mut ArrowTowerUpgrades>,
) {
    let mut tower_upgrades = upgrades
        .get_mut(tower)
        .expect("Provided Entity should exist / have the TowerUpgrades component");

    // Check if the tower already has the given upgrade
    if let Some(present_upgrade) = tower_upgrades.upgrades.iter_mut().find(|present_upgrade| {
        // Check that matches even if contained data is different
        ArrowTowerUpgradeDiscriminants::from(upgrade.clone()) == (*present_upgrade).clone().into()
    }) {
        // If possible, upgrade the level of the upgrade
        match present_upgrade {
            ArrowTowerUpgrade::Piercing(ref mut level)
            | ArrowTowerUpgrade::Multishot(ref mut level) => *level += 1.,

            ArrowTowerUpgrade::Tracking => {}
        }
    // Otherwise add the upgrade
    } else {
        tower_upgrades.upgrades.push(upgrade);
    }
}
