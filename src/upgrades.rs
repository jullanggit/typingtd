use bevy::prelude::*;

pub struct UpgradePlugin;
impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<Path>()
        //     .register_type::<(Direction, Path)>()
        //     .add_systems(Update, follow_path.after(apply_velocity))
        //     .add_systems(OnEnter(SpritesLoadingStates::Finished), load_path);
    }
}

trait TowerUpgrade {}
trait TowerUpgrades {}

#[derive(Debug, Clone, Reflect, PartialEq)]
pub enum ArrowTowerUpgrade {
    Piercing(f64),
    Multishot,
    Tracking,
}
impl TowerUpgrade for ArrowTowerUpgrade {}

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
#[repr(transparent)]
pub struct ArrowTowerUpgrades {
    pub upgrades: Vec<ArrowTowerUpgrade>,
}
impl TowerUpgrades for ArrowTowerUpgrades {}
