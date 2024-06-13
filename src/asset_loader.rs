use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

use crate::typing::Wordlists;

#[derive(Resource, Debug, Default, AssetCollection)]
pub struct Handles {
    #[asset(path = "all.words.json")]
    pub wordlists: Handle<Wordlists>,
    #[asset(path = "level1.png")]
    pub level1: Handle<Image>,
    #[asset(path = "dejavu-sans.book.ttf")]
    pub font: Handle<Font>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Handles>()
            .init_state::<SpritesLoadingStates>()
            .add_loading_state(
                LoadingState::new(SpritesLoadingStates::Loading)
                    .continue_to_state(SpritesLoadingStates::Finished)
                    .load_collection::<Handles>(),
            )
            .add_plugins(JsonAssetPlugin::<Wordlists>::new(&["words.json"]));
    }
}

#[derive(Clone, Eq, PartialEq, Default, States, Hash, Debug)]
pub enum SpritesLoadingStates {
    #[default]
    Loading,
    Finished,
}
