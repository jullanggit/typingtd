use bevy::prelude::*;

use crate::{map::MapAabb, quadtree::build_quadtree};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Velocity>()
            .register_type::<Position>()
            .register_type::<AABB>()
            .register_type::<Gravity>()
            .register_type::<MovingObjectState>()
            .register_type::<MovingObject>()
            .add_systems(
                Update,
                (update_physics, apply_gravity, collisions, stop_movement),
            );
    }
}

pub const GRAVITY_CONSTANT: f32 = 9.8;

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

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Gravity {
    pub force: f32,
    pub terminal_velocity: f32,
}

impl Gravity {
    pub const fn new(force: f32, terminal_velocity: f32) -> Self {
        Self {
            force,
            terminal_velocity,
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Translucent {
    slowdown: f32,
}
impl Default for Translucent {
    fn default() -> Self {
        Self { slowdown: 0.5 }
    }
}

#[derive(Component, Clone, Copy, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct MovingObjectState {
    pub right: bool,
    pub left: bool,
    pub ground: bool,
    pub ceiling: bool,
}

#[derive(Component, Clone, Copy, Reflect, Debug)]
#[reflect(Component)]
pub struct MovingObject {
    // timeless
    pub mass: f32,
    pub velocity_factor: f32,

    // current
    pub position: Position,
    pub velocity: Velocity,
    pub state: MovingObjectState,

    // old
    pub old_position: Position,
    pub old_velocity: Velocity,
    pub old_state: MovingObjectState,
}
impl Default for MovingObject {
    fn default() -> Self {
        Self {
            mass: 0.0,
            velocity_factor: 1.0,

            position: Position::default(),
            velocity: Velocity::default(),
            state: MovingObjectState::default(),

            old_position: Position::default(),
            old_velocity: Velocity::default(),
            old_state: MovingObjectState::default(),
        }
    }
}

#[derive(Bundle, Default)]
pub struct MovingObjectBundle {
    transform: Transform,
    aabb: AABB,
    moving_object: MovingObject,
    gravity: Gravity,
}

#[derive(Bundle, Default)]
pub struct MovingSpriteBundle {
    pub aabb: AABB,
    pub moving_object: MovingObject,
    pub sprite_bundle: SpriteBundle,
    pub gravity: Gravity,
}

#[derive(Bundle, Default)]
pub struct MovingSpriteSheetBundle {
    pub aabb: AABB,
    pub moving_object: MovingObject,
    pub spritesheet_bundle: SpriteSheetBundle,
    pub gravity: Gravity,
}

fn update_physics(mut query: Query<(&mut MovingObject, &mut Transform)>, time: Res<Time>) {
    for (mut moving_object, mut transform) in &mut query {
        moving_object.old_position = moving_object.position;
        moving_object.old_velocity = moving_object.velocity;
        moving_object.old_state = moving_object.state;

        let velocity_value = moving_object.velocity.value;
        let velocity_factor = moving_object.velocity_factor;
        moving_object.position.value += velocity_value * time.delta_seconds() * velocity_factor;

        transform.translation.x = moving_object.position.value.x;
        transform.translation.y = moving_object.position.value.y;
    }
}

fn stop_movement(mut query: Query<&mut MovingObject>) {
    for mut moving_object in &mut query {
        if (moving_object.state.ceiling || moving_object.state.ground)
            && (!moving_object.old_state.ceiling || !moving_object.old_state.ground)
        {
            moving_object.velocity.value.y = 0.0;
        }
        if (moving_object.state.left || moving_object.state.right)
            && (!moving_object.old_state.left || !moving_object.old_state.right)
        {
            moving_object.velocity.value.x = 0.0;
        }
    }
}

pub fn collisions(
    mut query: Query<(&AABB, &mut MovingObject, Entity, Option<&Translucent>)>,
    map_aabb: Res<MapAabb>,
) {
    // create quadtree
    let quadtree = build_quadtree(
        &query,
        &map_aabb.size,
        2,
        100,
        |(aabb, moving_object, entity, _)| (Some(aabb), moving_object, entity),
    );

    // create vec with all collisions to check
    let mut checks = Vec::new();

    // Iterate over all entities that have mass
    for (aabb, mut moving_object, entity, _) in &mut query {
        if moving_object.mass == 0.0 {
            continue;
        }
        // unset states
        moving_object.state.left = false;
        moving_object.state.right = false;
        moving_object.state.ground = false;
        moving_object.state.ceiling = false;
        moving_object.velocity_factor = 1.0;

        // create empty vec
        let mut to_check_collision = Vec::new();
        // add all entities to check against
        quadtree.query(aabb, moving_object.position, &mut to_check_collision);

        // remove duplicate entries
        to_check_collision.sort_unstable();
        to_check_collision.dedup();

        // construct vec with (original_entity, Vec<oter entites>)
        checks.push((entity, to_check_collision));
    }

    for (a_entity, others) in checks {
        for b_entity in others {
            // skip iteration if a and b are the same entity
            if a_entity == b_entity {
                continue;
            }

            // get components of both entities
            let [(a_aabb, mut a_moving_object, _, _), (b_aabb, mut b_moving_object, _, b_translucent)] =
                query.get_many_mut([a_entity, b_entity]).unwrap();

            // skip iteration if both objects have a mass of 0 (are stationary)
            if a_moving_object.mass == 0.0 && b_moving_object.mass == 0.0 {
                continue;
            }
            correct_collisions(
                &mut a_moving_object,
                a_aabb,
                &mut b_moving_object,
                b_aabb,
                b_translucent,
            );
        }
    }
}

fn correct_collisions(
    a_moving_object: &mut MovingObject,
    a_aabb: &AABB,
    b_moving_object: &mut MovingObject,
    b_aabb: &AABB,
    translucent: Option<&Translucent>,
) {
    let a_pos = a_moving_object.position;
    let b_pos = b_moving_object.position;

    // if there is a collision
    if let Some(penetration_depth) = penetration_depth(a_aabb, a_pos, b_aabb, b_pos) {
        match translucent {
            // Slow the entity down if it collides with something that is Translucent
            Some(translucent) => {
                a_moving_object.velocity_factor = translucent.slowdown;
            }
            // If it is a regular obstacle
            None => {
                let total_mass = a_moving_object.mass + b_moving_object.mass;
                let a_ratio = a_moving_object.mass / total_mass;
                let b_ratio = b_moving_object.mass / total_mass;

                // determine which axis to adjust
                if penetration_depth.x.abs() < penetration_depth.y.abs() {
                    // adjusting position
                    a_moving_object.position.value.x += penetration_depth.x * a_ratio;
                    b_moving_object.position.value.x -= penetration_depth.x * b_ratio;

                    // setting horizontal states
                    if penetration_depth.x >= 0.0 {
                        a_moving_object.state.left = true;
                        b_moving_object.state.right = true;
                    } else {
                        a_moving_object.state.right = true;
                        b_moving_object.state.left = true;
                    }
                } else {
                    // adjusting position
                    a_moving_object.position.value.y += penetration_depth.y * a_ratio;
                    b_moving_object.position.value.y -= penetration_depth.y * b_ratio;

                    if penetration_depth.y >= 0.0 {
                        a_moving_object.state.ground = true;
                        b_moving_object.state.ceiling = true;
                    } else {
                        a_moving_object.state.ceiling = true;
                        b_moving_object.state.ground = true;
                    }
                }
            }
        }
    }
}

fn apply_gravity(mut query: Query<(&mut MovingObject, &Gravity)>) {
    for (mut moving_object, gravity) in &mut query {
        if moving_object.state.ground {
            moving_object.velocity.value.y = 0.0;
        } else if moving_object.velocity.value.y > gravity.terminal_velocity {
            moving_object.velocity.value.y = gravity.terminal_velocity;
        } else if moving_object.velocity.value.y < -gravity.terminal_velocity {
            moving_object.velocity.value.y = -gravity.terminal_velocity;
        } else {
            moving_object.velocity.value.y -= gravity.force;
        }
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
