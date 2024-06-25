use bevy::prelude::*;
use strum::EnumIter;

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
pub struct Tower {
    pub tower_type: TowerType,
    pub priority: TowerPriority,
}
impl Tower {
    pub const fn new(tower_type: TowerType, priority: TowerPriority) -> Self {
        Self {
            tower_type,
            priority,
        }
    }
}

#[derive(Reflect, Clone, Copy, Debug)]
pub enum TowerType {
    Fire,
    Arrow,
}

#[derive(Reflect, Clone, Copy, Debug, EnumIter)]
pub enum TowerPriority {
    /// The enemy that is the nearest to the tower
    Nearest,
    /// The enemy that is the furthest on the path
    Furthest,
}

fn insert_tower_typing(
    towers: Query<(Entity, &Tower, &Position, &ArrowTowerUpgrades), Without<Children>>,
    mut commands: Commands,
    oneshot_systems: Res<OneShotSystems>,
) {
    for (entity, tower, position, arrow_tower_upgrades) in &towers {
        let action = match tower.tower_type {
            TowerType::Fire | TowerType::Arrow => {
                Action::ShootArrow(*position, arrow_tower_upgrades.clone(), tower.priority)
            }
        };
        commands.run_system_with_input(oneshot_systems.add_to_type, (entity, action, None));
    }
}

pub fn change_tower_priority(
    In((entity, priority)): In<(Entity, TowerPriority)>,
    mut towers: Query<&mut Tower>,
) {
    let mut tower = towers.get_mut(entity).expect("Provided tower should exist");
    tower.priority = priority;
}
