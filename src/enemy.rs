use crate::{
    asset_loader::{Handles, SpritesLoadingStates},
    map::{to_rgba_index, to_world, TILE_SIZE},
    physics::{apply_velocity, Layer, Position, Velocity},
};
use bevy::prelude::*;

const ENEMY_SPEED: f32 = 50.0;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Path>()
            .register_type::<Health>()
            .register_type::<Direction>()
            .register_type::<Path>()
            .register_type::<Enemy>()
            .add_systems(Update, follow_path)
            .add_systems(OnEnter(SpritesLoadingStates::Finished), load_path);
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Enemy {
    speed: f32,
}
impl Default for Enemy {
    fn default() -> Self {
        Self { speed: ENEMY_SPEED }
    }
}

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
    parts: Vec<Vec2>,
}

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct PathState {
    index: usize,
}

fn follow_path(
    mut query: Query<(&mut PathState, &Enemy, &mut Velocity, &mut Position)>,
    path: Res<Path>,
) {
    if path.parts.len() > 0 {
        query
            .iter_mut()
            .for_each(|(mut path_state, enemy, mut velocity, mut position)| {
                let direction =
                    to_0_or_1(path.parts[path_state.index] - path.parts[path_state.index - 1]);
                let remaining = path.parts[path_state.index] - position.value;

                let mult = to_0_or_1(remaining) * direction;
                if mult.x == -1.0 || mult.y == -1.0 {
                    position.value = path.parts[path_state.index];
                    path_state.index += 1;

                    // Recompute the direction, because of the index change
                    velocity.value =
                        to_0_or_1(path.parts[path_state.index] - path.parts[path_state.index - 1])
                            * enemy.speed;
                }
            });
    }
}

fn load_path(
    mut commands: Commands,
    mut path: ResMut<Path>,
    handles: Res<Handles>,
    images: Res<Assets<Image>>,
) {
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

    let mut index = to_rgba_index(start_image.0, start_image.1, image_size.x) as usize;
    let mut last_direction: Option<Direction> = None;
    loop {
        // If there is a tile above the current one, check for it being a path tile
        if last_direction != Some(Direction::Down)
            && let Some(above) = index.checked_sub(4 * image_size.x as usize)
            && image.data[above..above + 4] == [0, 0, 0, 255]
        {
            handle_insertion(
                &mut path,
                &mut index,
                above,
                Direction::Up,
                image_size,
                &mut last_direction,
            );
            continue;
        }
        // If there is a tile under the current one, check for it being a path tile
        let below = index + 4 * image_size.x as usize;
        if last_direction != Some(Direction::Up)
            && below < image.data.len()
            && image.data[below..below + 4] == [0, 0, 0, 255]
        {
            handle_insertion(
                &mut path,
                &mut index,
                below,
                Direction::Down,
                image_size,
                &mut last_direction,
            );
            continue;
        }
        // If there is a tile to the left of the current one (in the same row), check for it being a path tile
        if last_direction != Some(Direction::Right)
            && let Some(left) = index.checked_sub(4)
            && (left / 4) / image_size.x as usize == (index / 4) / image_size.x as usize
            && image.data[left..left + 4] == [0, 0, 0, 255]
        {
            handle_insertion(
                &mut path,
                &mut index,
                left,
                Direction::Left,
                image_size,
                &mut last_direction,
            );
            continue;
        }
        // If there is a tile to the left of the current one (in the same row), check for it being a path tile
        let right = index + 4;
        if last_direction != Some(Direction::Left)
            && right < image.data.len()
            && (right / 4) / image_size.x as usize == (index / 4) / image_size.x as usize
            && image.data[right..right + 4] == [0, 0, 0, 255]
        {
            handle_insertion(
                &mut path,
                &mut index,
                right,
                Direction::Right,
                image_size,
                &mut last_direction,
            );
            continue;
        }
        break;
    }
    dbg!(&path.parts);
    // TODO: handle enemy spawning
    commands.spawn((
        Name::new("Enemy"),
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            ..default()
        },
        Position::new(path.parts[0] - 2.0 * to_0_or_1(path.parts[1] - path.parts[0]) * TILE_SIZE),
        Velocity::new(to_0_or_1(path.parts[1] - path.parts[0]) * ENEMY_SPEED),
        Layer::new(3.0),
        Enemy { speed: ENEMY_SPEED },
        PathState { index: 1 },
    ));
}

fn handle_insertion(
    path: &mut Path,
    iter_index: &mut usize,
    current_index: usize,
    direction: Direction,
    size: UVec2,
    last_direction: &mut Option<Direction>,
) {
    *last_direction = Some(direction.clone());
    dbg!(&direction);

    let world_cords = to_world(
        (current_index / 4) as u32 % size.x,
        (current_index / 4) as u32 / size.x,
        size,
    );
    let parts_len = path.parts.len();
    // Check if the previous direction is the same, increment its length by tilesize if it
    // is, make a new part if it isnt
    if parts_len >= 2
        && to_0_or_1(path.parts[parts_len - 1] - path.parts[parts_len - 2]) == direction.into()
    {
        path.parts[parts_len - 1] = world_cords;
    } else {
        path.parts.push(world_cords);
    }
    *iter_index = current_index;
}

fn to_0_or_1(mut vec2: Vec2) -> Vec2 {
    if vec2.x != 0.0 {
        vec2.x /= vec2.x.abs()
    }
    if vec2.y != 0.0 {
        vec2.y /= vec2.y.abs()
    }
    vec2
}
