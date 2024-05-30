use bevy::prelude::*;

use crate::{oneshot::OneShotSystems, physics::Position, typing::Action};

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .register_type::<TowerType>()
            .add_systems(Update, insert_tower_typing);
    }
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Tower {
    pub tower_type: TowerType,
}

#[derive(Reflect, Clone, Debug)]
pub enum TowerType {
    Fire,
    Arrow,
}

fn insert_tower_typing(
    towers: Query<(Entity, &Tower, &Position), Without<Children>>,
    mut commands: Commands,
    oneshot_systems: Res<OneShotSystems>,
) {
    for (entity, tower, position) in &towers {
        let action = match tower.tower_type {
            TowerType::Fire | TowerType::Arrow => Action::ShootArrow(*position),
        };
        commands.run_system_with_input(oneshot_systems.add_to_type, (entity, action))
    }
}
