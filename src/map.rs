use crate::{
    asset_loader::Handles,
    physics::Position,
    states::GameState,
    tower::{Tower, TowerPriority, TowerType},
    upgrades::ArrowTowerUpgrades,
};
use bevy::prelude::*;

pub const TILE_SIZE: f32 = 32.0;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tile>()
            .register_type::<TileType>()
            .add_systems(OnExit(GameState::MainMenu), setup_map);
    }
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
#[repr(transparent)]
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

pub fn setup_map(
    mut commands: Commands,
    handles: Res<Handles>,
    images: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    // loading image and getting image size
    let level1_image = images.get(&handles.level1).expect("Image should be loaded");
    let size = level1_image.size();

    for x in 0..size.x {
        for y in 0..size.y {
            let pixel_index = to_rgba_index(x, y, size.x) as usize;
            let rgba = &level1_image.data[pixel_index..pixel_index + 4];

            match rgba {
                [0, 0, 0, 255] => spawn_tile(
                    &mut commands,
                    &handles,
                    "Way",
                    to_world(x, y, size),
                    TileType::Way,
                    83,
                ),
                [255, 255, 255, 255] => spawn_tile(
                    &mut commands,
                    &handles,
                    "Grass",
                    to_world(x, y, size),
                    TileType::Grass,
                    0,
                ),
                [111, 78, 55, 255] => spawn_tower(
                    &mut commands,
                    "Arrow Tower",
                    to_world(x, y, size),
                    TileType::Tower,
                    TowerType::Arrow,
                    asset_server.clone(),
                ),
                other => {
                    dbg!(other);
                }
            };
        }
    }
}

fn spawn_tile(
    commands: &mut Commands,
    handles: &Handles,
    name: &'static str,
    position: Vec2,
    tile_type: TileType,
    sprite_index: usize,
) {
    commands.spawn((
        Name::new(name),
        SpriteSheetBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            atlas: TextureAtlas {
                layout: handles.grass_layout.clone(),
                index: sprite_index,
            },
            texture: handles.grass.clone(),
            ..default()
        },
        Position::new(position),
        Tile { tile_type },
    ));
}

fn spawn_tower(
    commands: &mut Commands,
    name: &'static str,
    position: Vec2,
    tile_type: TileType,
    tower_type: TowerType,
    asset_server: AssetServer,
) {
    let tower: Handle<Image> = asset_server.load("tower.png");

    commands.spawn((
        Name::new(name),
        SpriteBundle {
            texture: tower,
            sprite: Sprite {
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            ..default()
        },
        Position::new(position),
        Tile { tile_type },
        Tower::new(tower_type, TowerPriority::Furthest),
        ArrowTowerUpgrades::default(),
    ));
}

pub fn to_world(x: u32, y: u32, size: UVec2) -> Vec2 {
    Vec2::new(
        (x as f32 - size.x as f32 / 2.0 + 0.5) * TILE_SIZE,
        -(y as f32 - size.y as f32 / 2.0 + 0.5) * TILE_SIZE,
    )
}
