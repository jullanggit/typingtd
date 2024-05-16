use bevy::prelude::*;

use crate::physics::{Rotation, Velocity};

pub const PROJECTILE_SPEED: f32 = 50000.0;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Projectile>()
            .add_systems(Update, move_projectile);
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Projectile {
    speed: f32,
}

impl Projectile {
    pub const fn new(speed: f32) -> Self {
        Self { speed }
    }
}

fn move_projectile(mut query: Query<(&Projectile, &Rotation, &mut Velocity)>, time: Res<Time>) {
    for (projectile, rotation, mut velocity) in &mut query {
        let forward = rotation.value * Vec3::X;
        velocity.value = forward.truncate() * projectile.speed * time.delta_seconds();
    }
}
