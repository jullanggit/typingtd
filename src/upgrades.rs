use std::fmt::Display;

use bevy::{prelude::*, utils::HashMap};
use strum::EnumIter;

use crate::enemy::Money;

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
            Self::Multishot => 30,
            Self::Tracking => 5,
        }
    }
    // TODO: Calculate cost properly
    pub fn cost(&self, level: u8) -> f64 {
        let five_plus_level = 3. + f64::from(level);
        five_plus_level * five_plus_level
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
    mut money: ResMut<Money>,
) {
    let mut tower_upgrades = upgrades
        .get_mut(tower)
        .expect("Provided Entity should exist / have the TowerUpgrades component");

    // Get the level of the upgrade, or insert the upgrade with a level of 1
    let level = tower_upgrades.upgrades.entry(upgrade.clone()).or_insert(0);
    let upgrade_cost = upgrade.cost(*level);

    if *level < upgrade.max_level() && money.value >= upgrade_cost {
        money.value -= upgrade_cost;
        *level += 1;
    } else if *level == 0{
        tower_upgrades.upgrades.remove(&upgrade);
    }
}
