use crate::{
    asset_loader::Handles,
    enemy::{Attack, Enemy, Health},
    path::PathState,
    physics::{Layer, Obb, Position, Rotation, Velocity, apply_velocity},
    tower::{Tower, TowerPriority},
    upgrades::{ArrowTowerUpgrade, ArrowTowerUpgrades},
};
use bevy::prelude::*;

pub const PROJECTILE_SPEED: f32 = 500.;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Projectile>()
            .register_type::<Tracking>()
            .add_systems(Update, track_enemy.before(apply_velocity))
            .add_observer(spawn_arrow);
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

#[derive(Debug, Clone, Event)]
pub struct SpawnArrow;

// Arrow Tower
/// Spawns an Arrow at the specified position, pointing towards the nearest Enemy
/// TODO: Make projectile speed an upgrade
pub fn spawn_arrow(
    trigger: Trigger<SpawnArrow>,
    tower_query: Query<(&Position, &ArrowTowerUpgrades, &Tower)>,
    enemies: Query<(&Position, &PathState), With<Enemy>>,
    mut commands: Commands,
    handles: Res<Handles>,
) {
    let (position, upgrades, tower) = tower_query
        .get(trigger.entity())
        .expect("Entity used to trigger this function should be in the query");
    let priority = tower.priority;

    // Get the closest enemy, exit if there arent any
    let Some(targeted_enemy) = (match priority {
        TowerPriority::Nearest => {
            closest_enemy(&enemies, *position, |(position, _)| position.value)
        }
        TowerPriority::Furthest => furthest_enemy(&enemies),
    }) else {
        return;
    };

    let direction_to_enemy_vec2 = (targeted_enemy - position.value).normalize();
    let direction_to_enemy = Quat::from_rotation_arc_2d(Vec2::X, direction_to_enemy_vec2);

    let shot_amount = upgrades[ArrowTowerUpgrade::Multishot];

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
            *position,
            Rotation::new(final_rotation),
            Projectile,
            Speed::new(PROJECTILE_SPEED),
            Sprite {
                image: handles.arrow.clone(),
                custom_size: Some(Vec2::new(45., 10.)),
                ..default()
            },
            Obb::new(Vec2::new(45., 10.)),
            Velocity::new((final_rotation * Vec3::X).truncate() * PROJECTILE_SPEED),
            Layer::new(1.),
            Attack::new(1.),
            Health::new(
                // Piercing value, or 1
                (upgrades[ArrowTowerUpgrade::Piercing] + 1).into(),
            ),
        ));
        if upgrades[ArrowTowerUpgrade::Tracking] > 0 {
            arrow.insert(Tracking::new(
                1.5 * f32::from(upgrades[ArrowTowerUpgrade::Tracking]),
            ));
        };
    }
}

fn closest_enemy<I, T, F>(enemies: I, arrow_position: Position, func: F) -> Option<Vec2>
where
    I: IntoIterator<Item = T>,
    F: Fn(T) -> Vec2,
{
    enemies
        .into_iter()
        .map(func)
        .min_by(|enemy_position1, enemy_position2| {
            arrow_position
                .value
                .distance(*enemy_position1)
                .total_cmp(&arrow_position.value.distance(*enemy_position2))
        })
}

// TODO: Make it target the furthest enemy, not just a rondom enemy of the ones that have the
// highest index
fn furthest_enemy(enemies: &Query<'_, '_, (&Position, &PathState), With<Enemy>>) -> Option<Vec2> {
    enemies
        .iter()
        .max_by_key(|&(_, path_state)| path_state.index)
        .map(|(position, _)| position.value)
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
        let Some(closest_enemy) =
            closest_enemy(&enemies, *arrow_position, |position| position.value)
        else {
            return;
        };

        // Get the rotation
        let direction = (closest_enemy - arrow_position.value).normalize();
        let target_rotation = Quat::from_rotation_arc_2d(Vec2::X, direction);

        // Calculate the rotation step based on the tracking speed and delta time
        let rotation_speed = tracking.rotation_speed * time.delta_secs();
        rotation.value = rotation.value.slerp(target_rotation, rotation_speed);

        // readjust the velocity
        velocity.value = (rotation.value * Vec3::X).truncate() * speed.value;
    }
}
