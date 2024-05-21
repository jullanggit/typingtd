use bevy::prelude::*;

use crate::{
    asset_loader::{Handles, SpritesLoadingStates},
    map::{to_rgba_index, TILE_SIZE},
};

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .register_type::<Direction>()
            .register_type::<Path>()
            .add_systems(OnEnter(SpritesLoadingStates::Finished), load_path);
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Health {
    max: f64,
    current: f64,
}

#[derive(PartialEq, Reflect, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
struct Path {
    parts: Vec<(Direction, f32)>,
    start: Vec2,
}

pub fn load_path(mut commands: Commands, handles: Res<Handles>, images: Res<Assets<Image>>) {
    // loading image and getting image size
    let image = images.get(&handles.level1).unwrap();
    let image_size = image.size();

    // Get the starting tile (in image coordinates)
    let start_image = (0..image_size.x)
        .flat_map(|x| (0..image_size.y).map(move |y| (x, y)))
        .find(|(x, y)| {
            let pixel_index = (y * image_size.x + x) as usize * 4; // Assuming 4 bytes per pixel (RGBA)
            let rgba = &image.data[pixel_index..pixel_index + 4];

            rgba == [0, 255, 0, 255]
        })
        .unwrap();
    // Get the world coordinates of the starting tile
    let start_world = Vec2::new(
        (start_image.0 as f32 - image_size.x as f32 / 2.0) * TILE_SIZE,
        (start_image.1 as f32 - image_size.y as f32 / 2.0) * TILE_SIZE,
    );

    let mut parts: Vec<(Direction, f32)> = Vec::new();

    let mut index = to_rgba_index(start_image.0, start_image.1, image_size.x) as usize;
    loop {
        // If there is a tile above the current one, check for it being a path tile
        if let Some(above) = index.checked_sub(image_size.x as usize)
            && image.data[above..above + 4] == [0, 0, 0, 255]
        {
            handle_insertion(parts.len(), &mut parts, &mut index, above, Direction::Up);
            continue;
        }
        // If there is a tile under the current one, check for it being a path tile
        let below = index + image_size.x as usize;
        if below < image.data.len() && image.data[below..below + 4] == [0, 0, 0, 255] {
            handle_insertion(parts.len(), &mut parts, &mut index, below, Direction::Down);
            continue;
        }
        // If there is a tile to the left of the current one (in the same row), check for it being a path tile
        if let Some(left) = index.checked_sub(1)
            && left / image_size.x as usize == index / image_size.x as usize
            && image.data[left..left + 4] == [0, 0, 0, 255]
        {
            handle_insertion(parts.len(), &mut parts, &mut index, left, Direction::Left);
            continue;
        }
        // If there is a tile to the left of the current one (in the same row), check for it being a path tile
        let right = index + 1;
        if right < image.data.len()
            && right / image_size.x as usize == index / image_size.x as usize
            && image.data[right..right + 4] == [0, 0, 0, 255]
        {
            handle_insertion(parts.len(), &mut parts, &mut index, right, Direction::Right);
            continue;
        }
        break;
    }
    commands.insert_resource(Path {
        start: start_world,
        parts,
    });
}

fn handle_insertion(
    parts_len: usize,
    parts: &mut Vec<(Direction, f32)>,
    index: &mut usize,
    other_index: usize,
    direction: Direction,
) {
    // Check if the previous direction is the same, increment its length by tilesize if it
    // is, make a new part if it isnt
    if parts_len > 0 && parts[parts_len - 1].0 == direction {
        parts[parts_len - 1].1 += TILE_SIZE;
    } else {
        parts.push((direction, TILE_SIZE));
    }
    *index = other_index;
}
