use bevy::prelude::*;

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .configure_sets(Update, GameSystemSet.run_if(in_state(GameState::Running)))
            .configure_sets(
                Update,
                LanguageMenuSystemSet.run_if(in_state(GameState::LanguageMenu)),
            )
            .configure_sets(
                Update,
                MainMenuSystemSet.run_if(in_state(GameState::MainMenu)),
            );
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Running,
    LanguageMenu,
    #[default]
    MainMenu,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameSystemSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LanguageMenuSystemSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MainMenuSystemSet;
