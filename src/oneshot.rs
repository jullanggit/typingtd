use bevy::{ecs::system::SystemId, prelude::*};

use crate::{physics::Position, projectile::Projectile, tower::spawn_arrow};

pub struct OneShotPlugin;
impl Plugin for OneShotPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OneShotSystems>();
    }
}

#[derive(Resource, Debug, Clone)]
pub struct OneShotSystems {
    pub spawn_arrow: SystemId<(Position, Projectile)>,
}
impl FromWorld for OneShotSystems {
    fn from_world(world: &mut World) -> Self {
        Self {
            spawn_arrow: world.register_system(spawn_arrow),
        }
    }
}
