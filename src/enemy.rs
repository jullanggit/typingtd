use bevy::prelude::*;
use strum::EnumIter;

use crate::{
    map::TILE_SIZE,
    path::{to_0_or_1, Path, PathState},
    physics::{Layer, Obb, Position, Rotation, Velocity},
    projectile::Speed,
};

pub const ENEMY_SPEED: f32 = 50.0;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .register_type::<Enemy>()
            .register_type::<Attack>()
            .add_systems(Update, (apply_damage, despawn_enemies).chain());
    }
}

#[derive(Component, Debug, Clone, Reflect, EnumIter)]
#[reflect(Component)]
pub enum Enemy {
    Base,
    Chunky,
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Attack {
    damage: f64,
}
impl Attack {
    pub const fn new(damage: f64) -> Self {
        Self { damage }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Health {
    max: f64,
    current: f64,
}
impl Health {
    pub const fn new(max: f64) -> Self {
        Self { max, current: max }
    }
}

fn apply_damage(
    mut attacks: Query<(&Position, &Rotation, &Obb, Option<&mut Health>, &Attack), Without<Enemy>>,
    mut enemies: Query<(&Position, &Rotation, &Obb, &mut Health), (With<Enemy>, Without<Attack>)>,
) {
    for (enemy_position, enemy_rotation, enemy_obb, mut enemy_health) in &mut enemies {
        for (attack_position, attack_rotation, attack_obb, mut attack_health_option, attack) in
            &mut attacks
        {
            if enemy_obb.collides(
                *enemy_position,
                enemy_rotation,
                attack_obb,
                *attack_position,
                attack_rotation,
            ) {
                if let Some(ref mut attack_health) = attack_health_option {
                    attack_health.current -= enemy_health.current;
                }
                enemy_health.current -= attack.damage;
            }
        }
    }
}

pub fn spawn_enemy(In(health): In<f64>, mut commands: Commands, path: Res<Path>) {
    commands.spawn((
        Name::new("Enemy"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            ..default()
        },
        Position::new(path.parts[0] - 2.0 * to_0_or_1(path.parts[1] - path.parts[0]) * TILE_SIZE),
        Velocity::new(to_0_or_1(path.parts[1] - path.parts[0]) * ENEMY_SPEED),
        Layer::new(3.0),
        Enemy::Base,
        Speed::new(ENEMY_SPEED),
        PathState::new(1),
        Rotation::default(),
        Obb::new(Vec2::splat(TILE_SIZE)),
        Health::new(health),
    ));
}

fn despawn_enemies(mut commands: Commands, enemies: Query<(&Health, Entity), With<Enemy>>) {
    for (health, entity) in &enemies {
        if health.current < 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
