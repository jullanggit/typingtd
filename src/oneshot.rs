use bevy::{ecs::system::SystemId, prelude::*};

use crate::{
    enemy::{spawn_enemy, Enemy},
    physics::Position,
    projectile::spawn_arrow,
    projectile::Speed,
    typing::{add_to_type, change_language, Action, Language},
    upgrades::ArrowTowerUpgrades,
};

pub struct OneShotPlugin;
impl Plugin for OneShotPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OneShotSystems>();
    }
}

#[derive(Resource, Debug, Clone)]
pub struct OneShotSystems {
    pub spawn_arrow: SystemId<(Position, Speed, ArrowTowerUpgrades)>,
    pub spawn_enemy: SystemId<Enemy>,
    pub add_to_type: SystemId<(Entity, Action, Option<String>)>,
    pub change_language: SystemId<Language>,
}
impl FromWorld for OneShotSystems {
    fn from_world(world: &mut World) -> Self {
        Self {
            spawn_arrow: world.register_system(spawn_arrow),
            spawn_enemy: world.register_system(spawn_enemy),
            add_to_type: world.register_system(add_to_type),
            change_language: world.register_system(change_language),
        }
    }
}
