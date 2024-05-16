use bevy::prelude::*;

use crate::physics::{Rotation, Velocity};

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Projectile {
    speed: f32,
}

impl Projectile {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

fn move_projectile(mut query: Query<(&Projectile, &Rotation, &mut Velocity)>, time: Res<Time>) {
    for (projectile, rotation, mut velocity) in &mut query {
        let forward = rotation.value * Vec3::Z;
        velocity.value = forward.truncate() * projectile.speed * time.delta_seconds();
    }
}
