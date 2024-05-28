use bevy::{prelude::*, render::camera::ScalingMode};

use crate::{
    asset_loader::{Handles, SpritesLoadingStates},
    map::TILE_SIZE,
};

// Plugin
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SpritesLoadingStates::Finished), spawn_camera);
    }
}

// Systems
fn spawn_camera(mut commands: Commands, images: Res<Assets<Image>>, handles: Res<Handles>) {
    let image = images
        .get(handles.level1.clone())
        .expect("Image should be loaded");

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: TILE_SIZE * image.width() as f32,
                height: TILE_SIZE * image.height() as f32,
            },
            far: -1000.0,
            near: 1000.0,
            ..default()
        },
        ..default()
    });
    // commands.spawn(Camera2dBundle::default());
}
