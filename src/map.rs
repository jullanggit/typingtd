use bevy::prelude::*;

use crate::{
    asset_loader::{Handles, SpritesLoadingStates},
    tower::{Tower, TowerType},
};

const TILE_SIZE: f32 = 32.0;

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

pub fn setup_map(mut commands: Commands, handles: Res<Handles>, images: Res<Assets<Image>>) {
    // loading image and getting image size
    let level1_image = images.get(&handles.level1).unwrap();
    let size = level1_image.size();

    for x in 0..size.x {
        for y in 0..size.y {
            let pixel_index = (y * level1_image.size().y + x) as usize * 4; // Assuming 4 bytes per pixel (RGBA)
            let rgba = &level1_image.data[pixel_index..pixel_index + 4];

            match rgba {
                [0, 0, 0, 255] => {
                    commands.spawn((
                        Name::new("Way"),
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                (x as f32 - size.x as f32 / 2.0) * TILE_SIZE,
                                (y as f32 - size.y as f32 / 2.0) * TILE_SIZE,
                                0.0,
                            ),
                            sprite: Sprite {
                                color: Color::rgba(
                                    rgba[0] as f32,
                                    rgba[1] as f32,
                                    rgba[2] as f32,
                                    rgba[3] as f32,
                                ),
                                custom_size: Some(Vec2::splat(TILE_SIZE as f32)),
                                ..default()
                            },
                            ..default()
                        },
                        Tile {
                            tile_type: TileType::Way,
                        },
                    ));
                }
                [255, 255, 255, 255] => {
                    commands.spawn((
                        Name::new("Grass"),
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                (x as f32 - size.x as f32 / 2.0) * TILE_SIZE,
                                (y as f32 - size.y as f32 / 2.0) * TILE_SIZE,
                                0.0,
                            ),
                            sprite: Sprite {
                                color: Color::rgba(
                                    rgba[0] as f32,
                                    rgba[1] as f32,
                                    rgba[2] as f32,
                                    rgba[3] as f32,
                                ),
                                custom_size: Some(Vec2::splat(TILE_SIZE as f32)),
                                ..default()
                            },
                            ..default()
                        },
                        Tile {
                            tile_type: TileType::Grass,
                        },
                    ));
                }
                [255, 0, 0, 255] => {
                    commands.spawn((
                        Name::new("Fire Tower"),
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                (x as f32 - size.x as f32 / 2.0) * TILE_SIZE,
                                (y as f32 - size.y as f32 / 2.0) * TILE_SIZE,
                                0.0,
                            ),
                            sprite: Sprite {
                                color: Color::rgba(
                                    rgba[0] as f32,
                                    rgba[1] as f32,
                                    rgba[2] as f32,
                                    rgba[3] as f32,
                                ),
                                custom_size: Some(Vec2::splat(TILE_SIZE as f32)),
                                ..default()
                            },
                            ..default()
                        },
                        Tile {
                            tile_type: TileType::Tower,
                        },
                        Tower {
                            tower_type: TowerType::Fire,
                        },
                    ));
                }
                other => {}
            };
        }
    }
}
