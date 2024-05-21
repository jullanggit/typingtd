#![feature(let_chains)]
#![allow(clippy::similar_names)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::needless_pass_by_value)]
// Conditionally compile the import for development builds only.
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use asset_loader::AssetLoaderPlugin;
use bevy::{asset::AssetMetaCheck, prelude::*};
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use fps::FpsPlugin;
use map::MapPlugin;
use oneshot::OneShotPlugin;
use physics::PhysicsPlugin;
use projectile::ProjectilePlugin;
use tower::TowerPlugin;
use typing::TypingPlugin;
#[cfg(target_family = "wasm")]
use wasm::WasmPlugin;

mod asset_loader;
mod camera;
mod enemy;
mod fps;
mod map;
mod oneshot;
mod physics;
mod projectile;
mod tower;
mod typing;
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
    app.add_plugins((
        CameraPlugin,
        AssetLoaderPlugin,
        TypingPlugin,
        MapPlugin,
        TowerPlugin,
        ProjectilePlugin,
        OneShotPlugin,
        PhysicsPlugin,
        EnemyPlugin,
    ));

    app.run();
}
