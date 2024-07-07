#![feature(let_chains)]
#![allow(clippy::similar_names)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::type_complexity)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::float_cmp)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]

#[cfg(not(target_family = "wasm"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use asset_loader::AssetLoaderPlugin;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use directors::DirectorPlugin;
use enemy::EnemyPlugin;
use map::MapPlugin;
use menus::MenuPlugin;
use path::PathPlugin;
use physics::PhysicsPlugin;
use projectile::ProjectilePlugin;
use sound::SoundPlugin;
use states::StatePlugin;
use tower::TowerPlugin;
use typing::TypingPlugin;
use upgrades::UpgradePlugin;
use wasm::WasmPlugin;

mod asset_loader;
mod camera;
mod directors;
mod enemy;
mod fps;
mod map;
mod menus;
mod path;
mod physics;
mod projectile;
mod sound;
mod states;
mod tower;
mod typing;
mod upgrades;
mod wasm;

fn main() {
    App::new()
        // disable checking for .meta files
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_debug_plugin(WorldInspectorPlugin::new())
        // app.add_debug_plugin(FpsPlugin)
        .add_wasm_plugin(WasmPlugin)
        // game plugins
        .add_plugins((
            CameraPlugin,
            AssetLoaderPlugin,
            TypingPlugin,
            MapPlugin,
            TowerPlugin,
            ProjectilePlugin,
            PhysicsPlugin,
            PathPlugin,
            EnemyPlugin,
            DirectorPlugin,
            MenuPlugin,
            UpgradePlugin,
            StatePlugin,
            SoundPlugin,
        ))
        .run();
}

trait DebugPlugin {
    fn add_debug_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self;
}
impl DebugPlugin for App {
    fn add_debug_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        #[cfg(debug_assertions)]
        self.add_plugins(plugin);
        self
    }
}

trait WasmPluginTrait {
    fn add_wasm_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self;
}
impl WasmPluginTrait for App {
    fn add_wasm_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        #[cfg(target_family = "wasm")]
        self.add_plugins(plugin);
        self
    }
}
