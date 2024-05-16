use bevy::prelude::*;

use crate::{
    asset_loader::Handles,
    enemy::Enemy,
    oneshot::OneShotSystems,
    physics::{Layer, Position, Rotation, Velocity},
    projectile::{Projectile, PROJECTILE_SPEED},
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
                    (*position, Projectile::new(PROJECTILE_SPEED)),
                ),
            }
            commands.entity(entity).remove::<ToType>();
        }
    }
}

fn insert_tower_typing(
    query: Query<(Entity, &Tower), Without<ToType>>,
    mut commands: Commands,
    wordlists: Res<Assets<Wordlists>>,
    handles: Res<Handles>,
    language: Res<Language>,
) {
    for (entity, tower) in &query {
        let word = wordlists
            .get(handles.wordlists.clone())
            .unwrap()
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

// Arrow Tower
/// Spawns an Arrow at the specified position, pointing towards the nearest Enemy
pub fn spawn_arrow(
    In((arrow_position, projectile)): In<(Position, Projectile)>,
    query: Query<(&Position), With<Enemy>>,
    mut commands: Commands,
) {
    let mut min_distance = f32::MAX;
    let mut closest_enemy_position = Vec2::ZERO;
    for enemy_position in &query {
        let distance = arrow_position.value.distance(enemy_position.value);
        if distance < min_distance {
            min_distance = distance;
            closest_enemy_position = enemy_position.value;
        }
    }

    let direction = (closest_enemy_position - arrow_position.value).normalize();
    let direction_quat = Quat::from_rotation_arc_2d(Vec2::X, direction);

    commands.spawn((
        Name::new("Arrow"),
        arrow_position,
        Rotation::new(direction_quat),
        projectile,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba_u8(68, 47, 47, 255),
                custom_size: Some(Vec2::new(90.0, 20.0)),
                ..default()
            },
            ..default()
        },
        Velocity::default(),
        Layer::new(1.0),
    ));
}
