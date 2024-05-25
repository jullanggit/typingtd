use bevy::prelude::*;

use crate::{
    asset_loader::Handles,
    oneshot::OneShotSystems,
    physics::Position,
    projectile::{Speed, PROJECTILE_SPEED},
    typing::{Action, Language, ToType, Wordlists},
};

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .register_type::<TowerType>()
            .add_systems(Update, (handle_tower_actions, insert_tower_typing));
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

fn handle_tower_actions(
    query: Query<(&Position, &ToType, Entity)>,
    mut commands: Commands,
    oneshot_systems: Res<OneShotSystems>,
) {
    for (position, to_type, entity) in &query {
        if to_type.progress >= to_type.word.len() {
            match to_type.action {
                // TODO: make the arrow shoot in the direction of the nearest enemy
                Action::ShootArrow => commands.run_system_with_input(
                    oneshot_systems.spawn_arrow,
                    (*position, Speed::new(PROJECTILE_SPEED)),
                ),
            }
            commands.entity(entity).remove::<ToType>();
        }
    }
}

fn insert_tower_typing(
    towers: Query<(Entity, &Tower), Without<ToType>>,
    mut commands: Commands,
    wordlists: Res<Assets<Wordlists>>,
    handles: Res<Handles>,
    language: Res<Language>,
) {
    for (entity, tower) in &towers {
        let word = wordlists
            .get(handles.wordlists.clone())
            .expect("Wordlists should be loaded")
            .get_word(&language);
        dbg!(&word);
        commands.entity(entity).insert(ToType::new(
            word,
            match tower.tower_type {
                TowerType::Arrow => Action::ShootArrow,
                TowerType::Fire => Action::ShootArrow,
            },
        ));
    }
}
