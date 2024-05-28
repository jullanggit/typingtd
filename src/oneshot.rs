use bevy::{ecs::system::SystemId, prelude::*};

use crate::{
    enemy::{spawn_enemy, Enemy},
    physics::Position,
    projectile::spawn_arrow,
    projectile::Speed,
};

pub struct OneShotPlugin;
impl Plugin for OneShotPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OneShotSystems>();
    }
}

#[derive(Resource, Debug, Clone)]
pub struct OneShotSystems {
    pub spawn_arrow: SystemId<(Position, Speed)>,
    pub spawn_enemy: SystemId<Enemy>,
}
impl FromWorld for OneShotSystems {
    fn from_world(world: &mut World) -> Self {
        Self {
            spawn_arrow: world.register_system(spawn_arrow),
            spawn_enemy: world.register_system(spawn_enemy),
        }
    }
}
