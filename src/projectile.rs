use crate::{
    enemy::{Attack, Enemy, Health},
    physics::{apply_velocity, Layer, Obb, Position, Rotation, Velocity},
    upgrades::{ArrowTowerUpgradeType, ArrowTowerUpgrades},
};
use bevy::prelude::*;

pub const PROJECTILE_SPEED: f32 = 500.;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Projectile>()
            .register_type::<Tracking>()
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
struct Tracking {
    rotation_speed: f32,
}
impl Tracking {
    const fn new(rotation_speed: f32) -> Self {
        Self { rotation_speed }
    }
}

// Arrow Tower
/// Spawns an Arrow at the specified position, pointing towards the nearest Enemy
pub fn spawn_arrow(
    In((arrow_position, speed, upgrades)): In<(Position, Speed, ArrowTowerUpgrades)>,
    enemies: Query<&Position, With<Enemy>>,
    mut commands: Commands,
) {
    // Get the closest enemy, exit if there arent any
    let Some(closest_enemy) = closest_enemy(&enemies, arrow_position) else {
        return;
    };

    let direction_to_enemy_vec2 = (closest_enemy - arrow_position.value).normalize();
    let direction_to_enemy = Quat::from_rotation_arc_2d(Vec2::X, direction_to_enemy_vec2);

    let shot_amount = upgrades
        .upgrades
        .get(&ArrowTowerUpgradeType::Multishot)
        .map_or(0, |amount| amount + 1);

    // Angle between arrows
    let arrow_angle = if shot_amount < 12 {
        30.
    } else {
        360. / f32::from(shot_amount)
    };
    for i in 0..=shot_amount {
        // Difference in rotation from the nearest enemy
        let i = f32::from(i);

        let angle = (i - f32::from(shot_amount) / 2.) * arrow_angle;
        let rotation_difference = Quat::from_rotation_z(angle.to_radians());

        let final_rotation = direction_to_enemy * rotation_difference;

        let mut arrow = commands.spawn((
            Name::new("Arrow"),
            arrow_position,
            Rotation::new(final_rotation),
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
            Velocity::new((final_rotation * Vec3::X).truncate() * speed.value),
            Layer::new(1.),
            Attack::new(1.),
            Health::new(
                // Piercing value, or 1
                upgrades
                    .upgrades
                    .get(&ArrowTowerUpgradeType::Piercing)
                    // Add two to the upgrade level, as base level is 0
                    .map_or(1., |upgrade| f64::from(upgrade + 2)),
            ),
        ));
        if let Some(level) = upgrades.upgrades.get(&ArrowTowerUpgradeType::Tracking) {
            arrow.insert(Tracking::new(1.5 * f32::from(level + 1)));
        };
    }
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
        (&Position, &Speed, &Tracking, &mut Rotation, &mut Velocity),
        Without<Enemy>,
    >,
    time: Res<Time>,
) {
    for (arrow_position, speed, tracking, mut rotation, mut velocity) in &mut tracking_arrows {
        // Get the closest enemy, exit if there arent any
        let Some(closest_enemy) = closest_enemy(&enemies, *arrow_position) else {
            return;
        };

        // Get the rotation
        let direction = (closest_enemy - arrow_position.value).normalize();
        let target_rotation = Quat::from_rotation_arc_2d(Vec2::X, direction);

        // Calculate the rotation step based on the tracking speed and delta time
        let rotation_speed = tracking.rotation_speed * time.delta_seconds();
        rotation.value = rotation.value.slerp(target_rotation, rotation_speed);

        // readjust the velocity
        velocity.value = (rotation.value * Vec3::X).truncate() * speed.value;
    }
}
