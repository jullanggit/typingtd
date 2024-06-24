use bevy::{ecs::system::SystemId, prelude::*};

use crate::{
    enemy::{spawn_enemy, Enemy},
    menus::{despawn_menus, spawn_menu},
    physics::Position,
    projectile::{spawn_arrow, Speed},
    states::{change_state, GameState},
    typing::{
        add_to_type, change_language, remove_inactive_to_types, toggle_to_type, Action, Language,
    },
    upgrades::{upgrade_tower, ArrowTowerUpgrade, ArrowTowerUpgrades},
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
    pub change_state: SystemId<GameState>,
    pub despawn_menus: SystemId,
    pub spawn_menu: SystemId<GameState>,
    pub toggle_to_type: SystemId,
    pub upgrade_tower: SystemId<(Entity, ArrowTowerUpgrade)>,
    pub remove_inactive_to_types: SystemId,
}
impl FromWorld for OneShotSystems {
    fn from_world(world: &mut World) -> Self {
        Self {
            spawn_arrow: world.register_system(spawn_arrow),
            spawn_enemy: world.register_system(spawn_enemy),
            add_to_type: world.register_system(add_to_type),
            change_language: world.register_system(change_language),
            change_state: world.register_system(change_state),
            despawn_menus: world.register_system(despawn_menus),
            spawn_menu: world.register_system(spawn_menu),
            toggle_to_type: world.register_system(toggle_to_type),
            upgrade_tower: world.register_system(upgrade_tower),
            remove_inactive_to_types: world.register_system(remove_inactive_to_types),
        }
    }
}
