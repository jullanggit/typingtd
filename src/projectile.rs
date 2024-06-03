use crate::{
    enemy::{Attack, Enemy},
    physics::{Layer, Obb, Position, Rotation, Velocity},
};
use bevy::prelude::*;

pub const PROJECTILE_SPEED: f32 = 500.;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Projectile>();
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

// Arrow Tower
/// Spawns an Arrow at the specified position, pointing towards the nearest Enemy
pub fn spawn_arrow(
    In((arrow_position, speed)): In<(Position, Speed)>,
    enemies: Query<&Position, With<Enemy>>,
    mut commands: Commands,
) {
    let Some(closest_enemy) =
        enemies
            .iter()
            .map(|position| position.value)
            .min_by(|enemy_position1, enemy_position2| {
                arrow_position
                    .value
                    .distance(*enemy_position1)
                    .total_cmp(&arrow_position.value.distance(*enemy_position2))
            })
    else {
        return;
    };

    let direction = (closest_enemy - arrow_position.value).normalize();
    let direction_quat = Quat::from_rotation_arc_2d(Vec2::X, direction);

    commands.spawn((
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
    ));
}
