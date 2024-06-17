use bevy::prelude::*;

use crate::{
    oneshot::OneShotSystems, physics::Position, states::GameSystemSet, typing::Action,
    upgrades::ArrowTowerUpgrades,
};

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .register_type::<TowerType>()
            .add_systems(Update, insert_tower_typing.in_set(GameSystemSet));
    }
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
#[repr(transparent)]
pub struct Tower {
    pub tower_type: TowerType,
}

#[derive(Reflect, Clone, Debug)]
pub enum TowerType {
    Fire,
    Arrow,
}

fn insert_tower_typing(
    towers: Query<(Entity, &Tower, &Position, &ArrowTowerUpgrades), Without<Children>>,
    mut commands: Commands,
    oneshot_systems: Res<OneShotSystems>,
) {
    for (entity, tower, position, arrow_tower_upgrades) in &towers {
        let action = match tower.tower_type {
            TowerType::Fire | TowerType::Arrow => {
                Action::ShootArrow(*position, arrow_tower_upgrades.clone())
            }
        };
        commands.run_system_with_input(oneshot_systems.add_to_type, (entity, action));
    }
}
