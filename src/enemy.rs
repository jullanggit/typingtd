use bevy::prelude::*;
use rand::{thread_rng, Rng};
use strum::{EnumCount, EnumIter};

use crate::{
    map::TILE_SIZE,
    path::{to_0_or_1, Path, PathState},
    physics::{Layer, Obb, Position, Rotation, Velocity},
    projectile::Speed,
    states::GameSystemSet,
};

pub const ENEMY_SPEED: f32 = 50.;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<(Health, Enemy, Attack)>()
            .init_resource::<Money>()
            .add_systems(
                Update,
                (
                    apply_damage,
                    despawn_enemies.after(apply_damage),
                    despawn_far_entities,
                )
                    .in_set(GameSystemSet),
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
    pub const fn credit_cost(&self) -> f64 {
        match self {
            Self::Base => 1.,
            Self::Chunky => 2.,
        }
    }
    pub const fn health(&self) -> f64 {
        match self {
            Self::Base => 1.,
            Self::Chunky => 3.,
        }
    }
    pub const fn value(&self) -> f64 {
        match self {
            Self::Base => 1.,
            Self::Chunky => 3.,
        }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
#[repr(transparent)]
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
#[repr(transparent)]
pub struct Health {
    value: f64,
}
impl Health {
    pub const fn new(value: f64) -> Self {
        Self { value }
    }
}

#[derive(Resource, Debug, Clone, Reflect, Default)]
#[reflect(Resource)]
#[repr(transparent)]
pub struct Money {
    value: f64,
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
                    attack_health.value -= enemy_health.value;
                }
                enemy_health.value -= attack.damage;
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
        Position::new(path.parts[0] - 2. * to_0_or_1(path.parts[1] - path.parts[0]) * TILE_SIZE),
        Velocity::new(to_0_or_1(path.parts[1] - path.parts[0]) * ENEMY_SPEED),
        Layer::new(3.),
        Health::new(variant.health()),
        variant,
        Speed::new(ENEMY_SPEED),
        PathState::new(1),
        Rotation::default(),
        Obb::new(Vec2::splat(TILE_SIZE)),
    ));
}

fn despawn_enemies(
    mut commands: Commands,
    enemies: Query<(&Health, &Enemy, Entity)>,
    mut money: ResMut<Money>,
) {
    for (health, enemy_type, entity) in &enemies {
        if health.value < 0. {
            money.value += enemy_type.value();
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_far_entities(
    entities: Query<(Entity, &Position, Option<(&Obb, &Rotation)>)>,
    camera: Query<&OrthographicProjection>,
    mut commands: Commands,
) {
    if let Ok(camera) = camera.get_single() {
        // Fix camera area not being set correctly for one frame after creation
        if camera.area.width() == 2. && camera.area.height() == 2. {
            return;
        }
        // Get the obb of the screen (camera) (with some lenience)
        let camera_obb = Obb::new(Vec2::new(
            camera.area.width() / 2. + TILE_SIZE,
            camera.area.height() / 2. + TILE_SIZE,
        ));

        // For every entity, check if it is off screen, despawn it if so
        for (entity, position, optional_stuff) in &entities {
            let in_window = match optional_stuff {
                Some((obb, rotation)) => obb.collides(
                    *position,
                    rotation,
                    &camera_obb,
                    Position::new(Vec2::ZERO),
                    &Rotation::new(Quat::IDENTITY),
                ),
                None => camera_obb.collides_point(
                    Position::new(Vec2::ZERO),
                    &Rotation::new(Quat::IDENTITY),
                    *position,
                ),
            };

            if !in_window {
                commands.entity(entity).despawn();
            }
        }
    }
}
