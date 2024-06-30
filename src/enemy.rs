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
        app.register_type::<Health>()
            .register_type::<Enemy>()
            .register_type::<Attack>()
            .init_resource::<Money>()
            .add_systems(
                Update,
                (
                    apply_damage,
                    despawn_dead_entities.after(apply_damage),
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
    pub value: f64,
}

fn apply_damage(
    mut enemies: Query<
        (&Position, &Rotation, &mut Obb, &mut Health, &mut Sprite),
        (With<Enemy>, Without<Attack>),
    >,
    mut attacks: Query<(&Position, &Rotation, &Obb, Option<&mut Health>, &Attack), Without<Enemy>>,
) {
    for (enemy_position, enemy_rotation, mut enemy_obb, mut enemy_health, mut enemy_sprite) in
        &mut enemies
    {
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

                // Adjust size based on hp
                let new_size = Vec2::splat(calculate_enemy_size(enemy_health.value as f32));
                enemy_obb.half_extents = new_size;
                enemy_sprite.custom_size = Some(new_size);
            }
        }
    }
}

pub fn spawn_enemy(
    In(variant): In<Enemy>,
    mut commands: Commands,
    path: Res<Path>,
    asset_server: Res<AssetServer>,
) {
    let enemy: Handle<Image> = asset_server.load("enemy.png");

    let size = Vec2::splat(calculate_enemy_size(variant.health() as f32));
    commands.spawn((
        Name::new(format!("{variant:?} Enemy")),
        SpriteBundle {
            texture: enemy,
            sprite: Sprite {
                custom_size: Some(size),
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
        Obb::new(size),
    ));
}
fn calculate_enemy_size(health: f32) -> f32 {
    f32::max(5.0, (TILE_SIZE * health) / 3.) // enemies sometimes not visible so size at least 5
}

fn despawn_dead_entities(
    mut commands: Commands,
    enemies: Query<(&Health, Option<&Enemy>, Entity)>,
    mut money: ResMut<Money>,
) {
    for (health, enemy_type, entity) in &enemies {
        if health.value <= 0. {
            if let Some(enemy_type) = enemy_type {
                money.value += enemy_type.value();
            }
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
