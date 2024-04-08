#![allow(clippy::similar_names)]
#![allow(clippy::module_name_repetitions)]
// Conditionally compile the import for development builds only.
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use asset_loader::AssetLoaderPlugin;
use bevy::{asset::AssetMetaCheck, prelude::*};
use camera::CameraPlugin;
use fps::FpsPlugin;
#[cfg(target_family = "wasm")]
use wasm::WasmPlugin;

mod asset_loader;
mod camera;
mod fps;
#[cfg(target_family = "wasm")]
mod wasm;

fn main() {
    let mut app = App::new();
    // disable checking for .meta files
    app.insert_resource(AssetMetaCheck::Never);

    // built-in plugins
    app.add_plugins(DefaultPlugins);

    // debug builds
    #[cfg(debug_assertions)]
    app.add_plugins(WorldInspectorPlugin::default());
    #[cfg(debug_assertions)]
    app.add_plugins(FpsPlugin);

    // wasm stuff
    #[cfg(target_family = "wasm")]
    app.add_plugins(WasmPlugin);

    // game plugins
    app.add_plugins((CameraPlugin, AssetLoaderPlugin));

    app.run();
}
