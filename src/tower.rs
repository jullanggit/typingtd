use std::cmp::Ordering;

use bevy::prelude::*;

use crate::{
    enemy::{self, Enemy},
    oneshot::OneShotSystems,
    physics::{Position, Rotation},
    projectile::Projectile,
    typing::{Action, ToType},
};

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>().register_type::<TowerType>();
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
    query: Query<(&Position, &ToType)>,
    mut commands: Commands,
    oneshot_systems: Res<OneShotSystems>,
) {
    for (position, to_type) in &query {
        if to_type.progress >= to_type.word.len() {
            match to_type.action {
                // TODO: make the arrow shoot in the direction of the nearest enemy
                Action::ShootArrow => commands.run_system_with_input(
                    oneshot_systems.spawn_arrow,
                    (*position, Projectile::new(100.0)),
                ),
            }
        }
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
    let to_closest_enemy_position =
        Quat::from_rotation_arc_2d(arrow_position.value, closest_enemy_position);

    commands.spawn((
        arrow_position,
        Rotation::new(to_closest_enemy_position),
        projectile,
        SpriteBundle::default(),
    ));
}
