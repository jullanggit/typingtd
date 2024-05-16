use bevy::prelude::*;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Velocity>()
            .register_type::<Position>()
            .register_type::<AABB>()
            .add_systems(Update, (apply_rotation))
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
}

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct AABB {
    pub halfsize: Vec2,
}
impl AABB {
    pub const fn new(halfsize: Vec2) -> Self {
        Self { halfsize }
    }

    pub fn contains<T1: AsRef<Vec2>, T2: AsRef<Vec2>>(
        &self,
        self_pos: T1,
        other: &Self,
        other_pos: T2,
    ) -> bool {
        let self_pos = self_pos.as_ref();
        let other_pos = other_pos.as_ref();
        // horizontal
        self_pos.x - self.halfsize.x < other_pos.x - other.halfsize.x
            && self_pos.x + self.halfsize.x > other_pos.x + other.halfsize.x
            // vertical
            && self_pos.y - self.halfsize.y < other_pos.y - other.halfsize.y
            && self_pos.y + self.halfsize.y > other_pos.y + other.halfsize.y
    }
}

fn apply_velocity(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in &mut query {
        position.value += velocity.value * time.delta_seconds();
    }
}
fn apply_position(mut query: Query<(&Position, &mut Transform)>) {
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

pub fn penetration_depth(
    a_aabb: &AABB,
    a_pos: Position,
    b_aabb: &AABB,
    b_pos: Position,
) -> Option<Vec2> {
    if collides(a_aabb, a_pos, b_aabb, b_pos) {
        let a_pos = a_pos.value;
        let b_pos = b_pos.value;

        let x = if a_pos.x > b_pos.x {
            (b_pos.x + b_aabb.halfsize.x) - (a_pos.x - a_aabb.halfsize.x)
        } else {
            (b_pos.x - b_aabb.halfsize.x) - (a_pos.x + a_aabb.halfsize.x)
        };
        let y = if a_pos.y > b_pos.y {
            (b_pos.y + b_aabb.halfsize.y) - (a_pos.y - a_aabb.halfsize.y)
        } else {
            (b_pos.y - b_aabb.halfsize.y) - (a_pos.y + a_aabb.halfsize.y)
        };

        return Some(Vec2::new(x, y));
    }
    None
}

pub fn collides(a_aabb: &AABB, a_pos: Position, b_aabb: &AABB, b_pos: Position) -> bool {
    let a_pos = a_pos.value;
    let b_pos = b_pos.value;

    (a_pos.x - a_aabb.halfsize.x) < (b_pos.x + b_aabb.halfsize.x)
        && (a_pos.x + a_aabb.halfsize.x) > (b_pos.x - b_aabb.halfsize.x)
        && (a_pos.y + a_aabb.halfsize.y) > (b_pos.y - b_aabb.halfsize.y)
        && (a_pos.y - a_aabb.halfsize.y) < (b_pos.y + b_aabb.halfsize.y)
}

pub fn intersects(a_aabb: &AABB, a_pos: Position, b_pos: Position) -> bool {
    let a_pos = a_pos.value;
    let b_pos = b_pos.value;

    (a_pos.x - a_aabb.halfsize.x) < b_pos.x
        && (a_pos.x + a_aabb.halfsize.x) > b_pos.x
        && (a_pos.y + a_aabb.halfsize.y) > b_pos.y
        && (a_pos.y - a_aabb.halfsize.y) < b_pos.y
}
