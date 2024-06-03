use bevy::prelude::*;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<(Velocity, Position, Rotation, Layer)>()
            .add_systems(Update, (apply_rotation, apply_layer))
            .add_systems(Update, (apply_velocity, apply_position).chain());
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
#[reflect(Component)]
pub struct Velocity {
    pub value: Vec2,
}
impl Velocity {
    pub const fn new(value: Vec2) -> Self {
        Self { value }
    }
}

#[derive(Component, Clone, Copy, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct Position {
    pub value: Vec2,
}
impl AsRef<Vec2> for Position {
    fn as_ref(&self) -> &Vec2 {
        &self.value
    }
}

impl Position {
    pub const fn new(value: Vec2) -> Self {
        Self { value }
    }
}

#[derive(Component, Clone, Copy, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct Rotation {
    pub value: Quat,
}
impl Rotation {
    pub const fn new(value: Quat) -> Self {
        Self { value }
    }
    fn compute_axes(&self) -> [Vec2; 2] {
        let x_axis = self.value * Vec3::X;
        let y_axis = self.value * Vec3::Y;
        [x_axis.truncate(), y_axis.truncate()]
    }
}

// To be used with Position and Rotation Component
#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Obb {
    pub half_extents: Vec2,
}
impl Obb {
    pub const fn new(half_extents: Vec2) -> Self {
        Self { half_extents }
    }
    pub fn collides(
        &self,
        self_center: Position,
        self_rotation: &Rotation,
        other_obb: &Self,
        other_center: Position,
        other_rotation: &Rotation,
    ) -> bool {
        let axes_nested = [self_rotation.compute_axes(), other_rotation.compute_axes()];
        let axes = axes_nested.as_flattened();
        let center_delta = other_center.value - self_center.value;

        !axes
            .iter()
            .any(|&axis| !self.overlap_on_axis(axis, center_delta, other_obb, axes))
    }
    pub fn collides_point(
        &self,
        self_center: Position,
        self_rotation: &Rotation,
        point: Position,
    ) -> bool {
        let local_axes = self_rotation.compute_axes();
        let local_point = point.value - self_center.value;

        // Transform the point into the OBB's local space
        let local_x = local_point.dot(local_axes[0]);
        let local_y = local_point.dot(local_axes[1]);

        // Check if the point is within the OBB's extents
        local_x.abs() <= self.half_extents.x && local_y.abs() <= self.half_extents.y
    }
    fn overlap_on_axis(
        &self,
        axis: Vec2,
        center_delta: Vec2,
        other_obb: &Self,
        axes: &[Vec2],
    ) -> bool {
        let center_projection = center_delta.dot(axis).abs();

        // More accurate/performant than normal mult + add
        let a_projection = self.half_extents.x.mul_add(
            axes[0].dot(axis).abs(),
            self.half_extents.y * axes[1].dot(axis).abs(),
        );
        let b_projection = other_obb.half_extents.x.mul_add(
            axes[2].dot(axis).abs(),
            other_obb.half_extents.y * axes[3].dot(axis).abs(),
        );

        center_projection <= a_projection + b_projection
    }
}

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Layer {
    value: f32,
}
impl Layer {
    pub const fn new(value: f32) -> Self {
        Self { value }
    }
}

pub fn apply_velocity(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in &mut query {
        position.value += velocity.value * time.delta_seconds();
    }
}
pub fn apply_position(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in &mut query {
        transform.translation.x = position.value.x;
        transform.translation.y = position.value.y;
    }
}
fn apply_rotation(mut query: Query<(&Rotation, &mut Transform)>) {
    for (rotation, mut transform) in &mut query {
        transform.rotation = rotation.value;
    }
}
fn apply_layer(mut query: Query<(&Layer, &mut Transform)>) {
    for (layer, mut transform) in &mut query {
        transform.translation.z = layer.value;
    }
}
