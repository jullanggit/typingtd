use bevy::prelude::*;

pub struct StatesPlugin;
impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .configure_sets(Update, GameSystemSet.run_if(in_state(GameState::Running)));
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Running,
    LanguageMenu,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameSystemSet;
