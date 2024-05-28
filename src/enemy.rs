use bevy::prelude::*;
use rand::{thread_rng, Rng};
use strum::{EnumCount, EnumIter};

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
            .add_systems(
                Update,
                (
                    apply_damage,
                    despawn_enemies.after(apply_damage),
                    despawn_far_entities,
                ),
            );
    }
}

#[derive(Component, Debug, Clone, Reflect, EnumIter, EnumCount)]
#[reflect(Component)]
pub enum Enemy {
    Base,
    Chunky,
}
impl Enemy {
    pub fn random() -> Self {
        match thread_rng().gen_range(0..Self::COUNT) {
            0 => Self::Base,
            1 => Self::Chunky,
            _ => unreachable!(),
        }
    }
    pub const fn cost(&self) -> f64 {
        match self {
            Self::Base => 1.0,
            Self::Chunky => 2.0,
        }
    }
    pub const fn health(&self) -> f64 {
        match self {
            Self::Base => 1.0,
            Self::Chunky => 3.0,
        }
    }
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

pub fn spawn_enemy(In(variant): In<Enemy>, mut commands: Commands, path: Res<Path>) {
    commands.spawn((
        Name::new(format!("{variant:?} Enemy")),
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
        Health::new(variant.health()),
        variant,
        Speed::new(ENEMY_SPEED),
        PathState::new(1),
        Rotation::default(),
        Obb::new(Vec2::splat(TILE_SIZE)),
    ));
}

fn despawn_enemies(mut commands: Commands, enemies: Query<(&Health, Entity), With<Enemy>>) {
    for (health, entity) in &enemies {
        if health.current < 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_far_entities(
    entities: Query<(Entity, &Position, Option<(&Obb, &Rotation)>)>,
    camera: Query<&OrthographicProjection>,
    mut commands: Commands,
) {
    // Get the obb of the screen (camera) (with some lenience)
    let camera_area = camera.single().area;
    let camera_obb = Obb::new(Vec2::new(
        camera_area.width() / 2.0 + TILE_SIZE,
        camera_area.height() / 2.0 + TILE_SIZE,
    ));

    for (entity, position, optional_stuff) in &entities {
        let not_in_window = match optional_stuff {
            Some((obb, rotation)) => !obb.collides(
                *position,
                rotation,
                &camera_obb,
                Position::new(Vec2::ZERO),
                &Rotation::new(Quat::IDENTITY),
            ),
            None => camera_obb.collides_point(Position::new(Vec2::ZERO), *position),
        };

        if not_in_window {
            commands.entity(entity).despawn()
        }
    }
}
