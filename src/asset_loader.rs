use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

use crate::{states::GameState, typing::Wordlists};

#[derive(Resource, Debug, Default, AssetCollection)]
pub struct Handles {
    #[asset(path = "all.words.json")]
    pub wordlists: Handle<Wordlists>,

    #[asset(path = "level1.png")]
    pub level1: Handle<Image>,

    #[asset(path = "level2.png")]
    pub level2: Handle<Image>,

    #[asset(path = "level3.png")]
    pub level3: Handle<Image>,

    #[asset(path = "level4.png")]
    pub level4: Handle<Image>,

    #[asset(path = "Normaleste.ttf")]
    pub font: Handle<Font>,

    #[asset(path = "grass.png")]
    #[asset(image(sampler = nearest))]
    pub grass: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 16., tile_size_y = 16., columns = 25, rows = 14))]
    pub grass_layout: Handle<TextureAtlasLayout>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Handles>()
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::MainMenu)
                    .load_collection::<Handles>(),
            )
            .add_plugins(JsonAssetPlugin::<Wordlists>::new(&["words.json"]));
    }
}
