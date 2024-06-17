use bevy::prelude::*;

#[derive(Debug, Clone, Reflect, PartialEq)]
pub enum ArrowTowerUpgrade {
    Piercing(f64),
    Multishot,
    Tracking,
}

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
#[repr(transparent)]
pub struct ArrowTowerUpgrades {
    pub upgrades: Vec<ArrowTowerUpgrade>,
}
