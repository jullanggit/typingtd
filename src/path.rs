use crate::{
    asset_loader::Handles,
    enemy::Enemy,
    map::{to_rgba_index, to_world},
    physics::{apply_velocity, Position, Velocity},
    projectile::Speed,
    states::{GameState, GameSystemSet},
};
use bevy::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

pub struct PathPlugin;
impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Path>()
            .register_type::<Path>()
            .register_type::<Direction>()
            .add_systems(
                Update,
                (follow_path.after(apply_velocity)).in_set(GameSystemSet),
            )
            .add_systems(OnExit(GameState::MainMenu), load_path);
    }
}

#[derive(PartialEq, Reflect, Clone, Debug, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    const fn offset(&self, image_size: UVec2) -> isize {
        match self {
            Self::Up => -(4 * image_size.x as isize),
            Self::Down => 4 * image_size.x as isize,
            Self::Left => -4,
            Self::Right => 4,
        }
    }
    const fn inverse(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}
impl From<Direction> for Vec2 {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Up => Self::Y,
            Direction::Down => -Self::Y,
            Direction::Left => -Self::X,
            Direction::Right => Self::X,
        }
    }
}

#[derive(Resource, Debug, Clone, Reflect, Default)]
#[reflect(Resource)]
#[repr(transparent)]
pub struct Path {
    pub parts: Vec<Vec2>,
}

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
#[repr(transparent)]
pub struct PathState {
    pub index: usize,
}
impl PathState {
    pub const fn new(index: usize) -> Self {
        Self { index }
    }
}

fn follow_path(
    mut enemies: Query<(&mut PathState, &Speed, &mut Velocity, &mut Position), With<Enemy>>,
    path: Res<Path>,
) {
    for (mut path_state, speed, mut velocity, mut position) in &mut enemies {
        if path_state.index < path.parts.len() - 1 {
            let direction =
                to_0_or_1(path.parts[path_state.index] - path.parts[path_state.index - 1]);
            let remaining = path.parts[path_state.index] - position.value;

            let mult = to_0_or_1(remaining) * direction;
            if mult.x == -1. || mult.y == -1. {
                position.value = path.parts[path_state.index];
                path_state.index += 1;

                // Recompute the direction, because of the index change
                velocity.value =
                    to_0_or_1(path.parts[path_state.index] - path.parts[path_state.index - 1])
                        * speed.value;
            }
        }
    }
}

fn load_path(mut path: ResMut<Path>, handles: Res<Handles>, images: Res<Assets<Image>>) {
    // loading image and getting image size
    let image = images.get(&handles.level1).expect("Image should be loaded");
    let image_size = image.size();

    // Get the starting tile (in image coordinates)
    let start_image = find_start_tile(image_size, image).expect("Path start should exist");

    let mut last_index = to_rgba_index(start_image.0, start_image.1, image_size.x) as usize;
    let mut last_direction: Option<Direction> = None;

    // While there is a next path tile, either update the last segment, or make a new one
    while let Some((next_index, current_direction)) =
        find_next_index(image, last_index, &last_direction)
    {
        let world_cords = to_world(
            (next_index / 4) as u32 % image_size.x,
            (next_index / 4) as u32 / image_size.x,
            image_size,
        );
        let parts_len = path.parts.len();

        // If the last direction is the same as the current one
        if parts_len > 1 && last_direction == Some(current_direction.clone()) {
            path.parts[parts_len - 1] = world_cords;
        } else {
            path.parts.push(world_cords);
        }

        last_direction = Some(current_direction);
        last_index = next_index;
    }
}

fn find_start_tile(image_size: UVec2, image: &Image) -> Option<(u32, u32)> {
    (0..image_size.x)
        .flat_map(|x| (0..image_size.y).map(move |y| (x, y)))
        .find(|(x, y)| {
            let pixel_index = (y * image_size.x + x) as usize * 4; // Assuming 4 bytes per pixel (RGBA)
            let rgba = &image.data[pixel_index..pixel_index + 4];

            rgba == [0, 255, 0, 255]
        })
}

fn find_next_index(
    image: &Image,
    index: usize,
    last_direction: &Option<Direction>,
) -> Option<(usize, Direction)> {
    let image_size = image.size();

    for direction in Direction::iter() {
        if *last_direction == Some(direction.inverse()) {
            continue;
        }

        // Calculate the next index, return none if it would be < 0
        let Ok(next_index) = (index as isize + direction.offset(image.size())).try_into() else {
            continue;
        };

        // If direction is left or right, check if the tile is on the same row as the previous one
        let is_valid = match direction {
            Direction::Left | Direction::Right => {
                (index / 4) / image_size.x as usize == (next_index / 4) / image_size.x as usize
            }
            _ => true,
        };

        // Also check if the index is inside the images bounds, and if the tile is a path tile
        if is_valid
            && next_index < image.data.len()
            && image.data[next_index..next_index + 4] == [0, 0, 0, 255]
        {
            return Some((next_index, direction));
        }
    }

    None
}

pub fn to_0_or_1(mut vec2: Vec2) -> Vec2 {
    if vec2.x != 0. {
        vec2.x /= vec2.x.abs();
    }
    if vec2.y != 0. {
        vec2.y /= vec2.y.abs();
    }
    vec2
}
