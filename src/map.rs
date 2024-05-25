use bevy::prelude::*;

use crate::{
    asset_loader::{Handles, SpritesLoadingStates},
    physics::Position,
    tower::{Tower, TowerType},
};

pub const TILE_SIZE: f32 = 32.0;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tile>()
            .register_type::<TileType>()
            .add_systems(OnEnter(SpritesLoadingStates::Finished), setup_map);
    }
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
struct Tile {
    tile_type: TileType,
}

#[derive(Reflect, Clone, Debug)]
enum TileType {
    Grass,
    Way,
    Tower,
}

pub const fn to_rgba_index(x: u32, y: u32, width: u32) -> u32 {
    (y * width + x) * 4
}

pub fn setup_map(mut commands: Commands, handles: Res<Handles>, images: Res<Assets<Image>>) {
    // loading image and getting image size
    let level1_image = images.get(&handles.level1).expect("Image should be loaded");
    let size = level1_image.size();

    for x in 0..size.x {
        for y in 0..size.y {
            let pixel_index = to_rgba_index(x, y, size.x) as usize;
            let rgba = &level1_image.data[pixel_index..pixel_index + 4];

            let mut entity = commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba_u8(rgba[0], rgba[1], rgba[2], rgba[3]),
                        custom_size: Some(Vec2::splat(TILE_SIZE)),
                        ..default()
                    },
                    ..default()
                },
                Position::new(to_world(x, y, size)),
            ));

            match rgba {
                [0, 0, 0, 255] => {
                    entity.insert((
                        Name::new("Way"),
                        Tile {
                            tile_type: TileType::Way,
                        },
                    ));
                }
                [255, 255, 255, 255] => {
                    entity.insert((
                        Name::new("Grass"),
                        Tile {
                            tile_type: TileType::Grass,
                        },
                    ));
                }
                // [255, 0, 0, 255] => {
                //     entity.insert((
                //         Name::new("Fire Tower"),
                //         Tile {
                //             tile_type: TileType::Tower,
                //         },
                //         Tower {
                //             tower_type: TowerType::Fire,
                //         },
                //     ));
                // }
                [111, 78, 55, 255] => {
                    entity.insert((
                        Name::new("Arrow Tower"),
                        Tile {
                            tile_type: TileType::Tower,
                        },
                        Tower {
                            tower_type: TowerType::Arrow,
                        },
                    ));
                }
                other => {
                    dbg!(other);
                }
            };
        }
    }
}

pub fn to_world(x: u32, y: u32, size: UVec2) -> Vec2 {
    Vec2::new(
        (x as f32 - size.x as f32 / 2.0) * TILE_SIZE,
        -(y as f32 - size.y as f32 / 2.0) * TILE_SIZE,
    )
}
