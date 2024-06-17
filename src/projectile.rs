use crate::{
    enemy::{Attack, Enemy, Health},
    physics::{apply_velocity, Layer, Obb, Position, Rotation, Velocity},
    upgrades::{ArrowTowerUpgrade, ArrowTowerUpgrades},
};
use bevy::prelude::*;

pub const PROJECTILE_SPEED: f32 = 500.;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Projectile>()
            .add_systems(Update, track_enemy.before(apply_velocity));
    }
}

#[derive(Component, Debug, Copy, Clone, Reflect)]
#[reflect(Component)]
#[repr(transparent)]
pub struct Speed {
    pub value: f32,
}
impl Speed {
    pub const fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Projectile;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
struct Tracking;

// Arrow Tower
/// Spawns an Arrow at the specified position, pointing towards the nearest Enemy
/// TODO: implement multishot
pub fn spawn_arrow(
    In((arrow_position, speed, upgrades)): In<(Position, Speed, ArrowTowerUpgrades)>,
    enemies: Query<&Position, With<Enemy>>,
    mut commands: Commands,
) {
    // Get the closest enemy, exit if there arent any
    let Some(closest_enemy) = closest_enemy(&enemies, arrow_position) else {
        return;
    };

    let direction = (closest_enemy - arrow_position.value).normalize();
    let direction_quat = Quat::from_rotation_arc_2d(Vec2::X, direction);

    let mut arrow = commands.spawn((
        Name::new("Arrow"),
        arrow_position,
        Rotation::new(direction_quat),
        Projectile,
        speed,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba_u8(68, 47, 47, 255),
                custom_size: Some(Vec2::new(45., 10.)),
                ..default()
            },
            ..default()
        },
        Obb::new(Vec2::new(45., 10.)),
        Velocity::new((direction_quat * Vec3::X).truncate() * speed.value),
        Layer::new(1.),
        Attack::new(1.),
        Health::new(
            // Piercing value, or 1
            upgrades
                .upgrades
                .iter()
                .find_map(|upgrade| match upgrade {
                    ArrowTowerUpgrade::Piercing(value) => Some(*value),
                    _ => None,
                })
                .unwrap_or(1.),
        ),
    ));
    if upgrades.upgrades.contains(&ArrowTowerUpgrade::Tracking) {
        arrow.insert(Tracking);
    };
}

fn closest_enemy(
    enemies: &Query<'_, '_, &Position, With<Enemy>>,
    arrow_position: Position,
) -> Option<Vec2> {
    enemies
        .iter()
        .map(|position| position.value)
        .min_by(|enemy_position1, enemy_position2| {
            arrow_position
                .value
                .distance(*enemy_position1)
                .total_cmp(&arrow_position.value.distance(*enemy_position2))
        })
}

fn track_enemy(
    enemies: Query<&Position, With<Enemy>>,
    mut tracking_arrows: Query<
        (&Position, &Speed, &mut Rotation, &mut Velocity),
        (With<Tracking>, Without<Enemy>),
    >,
) {
    for (arrow_position, speed, mut rotation, mut velocity) in &mut tracking_arrows {
        // Get the closest enemy, exit if there arent any
        let Some(closest_enemy) = closest_enemy(&enemies, *arrow_position) else {
            return;
        };

        // Get the rotation
        let direction = (closest_enemy - arrow_position.value).normalize();
        let direction_quat = Quat::from_rotation_arc_2d(Vec2::X, direction);

        rotation.value = direction_quat;

        // readjust the velocity
        velocity.value = (direction_quat * Vec3::X).truncate() * speed.value;
    }
}
