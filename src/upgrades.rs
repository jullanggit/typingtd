use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use bevy::prelude::*;
use strum::{EnumCount, EnumIter};

use crate::{
    enemy::Money,
    states::PauseMenuSystemSet,
    typing::{Action, ToType},
};

pub struct UpgradePlugin;
impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ArrowTowerUpgrades>()
            .add_systems(Update, update_upgrade_price.in_set(PauseMenuSystemSet))
            .observe(upgrade_tower);
    }
}

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq, Hash, EnumIter, EnumCount)]
pub enum ArrowTowerUpgrade {
    Piercing,
    Multishot,
    Tracking,
}
impl ArrowTowerUpgrade {
    const fn max_level(self) -> u8 {
        match self {
            Self::Piercing => u8::MAX,
            Self::Multishot => 30,
            Self::Tracking => 5,
        }
    }
    // TODO: Calculate cost properly
    pub fn cost(self, level: u8) -> f64 {
        let five_plus_level = 3. + f64::from(level);
        five_plus_level * five_plus_level
    }
}

impl Display for ArrowTowerUpgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
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
    upgrades: [u8; ArrowTowerUpgrade::COUNT],
}
impl Index<ArrowTowerUpgrade> for ArrowTowerUpgrades {
    type Output = u8;
    fn index(&self, index: ArrowTowerUpgrade) -> &Self::Output {
        &self.upgrades[index as usize]
    }
}
impl IndexMut<ArrowTowerUpgrade> for ArrowTowerUpgrades {
    fn index_mut(&mut self, index: ArrowTowerUpgrade) -> &mut Self::Output {
        &mut self.upgrades[index as usize]
    }
}

#[derive(Debug, Clone, Event)]
pub struct UpgradeTower(pub ArrowTowerUpgrade);

pub fn upgrade_tower(
    trigger: Trigger<UpgradeTower>,
    mut upgrades: Query<&mut ArrowTowerUpgrades>,
    mut money: ResMut<Money>,
) {
    let upgrade = trigger.event().0;

    let mut tower_upgrades = upgrades
        .get_mut(trigger.entity())
        .expect("Provided Entity should exist / have the TowerUpgrades component");

    // Get the level of the upgrade, or insert the upgrade with a level of 1
    let level = tower_upgrades[upgrade];
    let upgrade_cost = upgrade.cost(level);

    if level < upgrade.max_level() && money.value >= upgrade_cost {
        money.value -= upgrade_cost;
        tower_upgrades[upgrade] += 1;
    }
}

pub fn update_upgrade_price(
    mut to_types: Query<&mut ToType>,
    upgrades: Query<&ArrowTowerUpgrades>,
) {
    for mut to_type in &mut to_types {
        if let Action::UpgradeTower(entity, upgrade) = to_type.action {
            // Get the level of the upgrade
            let level = upgrades
                .get(entity)
                .expect("Passed entity should have Upgrades")[upgrade];

            let cost = upgrade.cost(level);

            to_type.word = format!("{upgrade} {cost}$");
        }
    }
}
