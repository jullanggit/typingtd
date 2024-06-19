use std::fmt::Display;

use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::typing::{Action, Language};

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .configure_sets(Update, GameSystemSet.run_if(not_in_menu_state))
            .configure_sets(Update, PauseMenuSystemSet.run_if(in_menu_state));
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum GameState {
    #[default]
    Running,
    PauseMenu,
    LanguageMenu,
}
impl GameState {
    /// If the State is a menu state, returns a the actions for the buttons in the menu
    pub fn get_buttons(&self) -> Option<Vec<Action>> {
        match self {
            Self::Running => None,
            Self::PauseMenu => Some(vec![Action::ChangeMenu(Self::LanguageMenu)]),
            Self::LanguageMenu => Some(Language::iter().map(Action::ChangeLanguage).collect()),
        }
    }
    pub const fn is_menu_state(&self) -> bool {
        match self {
            Self::Running => false,
            _other => true,
        }
    }
}
impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Running => "Running",
                Self::PauseMenu => "Options",
                Self::LanguageMenu => "Languages",
            }
        )
    }
}

fn in_menu_state(state: Res<State<GameState>>) -> bool {
    state.is_menu_state()
}
fn not_in_menu_state(state: Res<State<GameState>>) -> bool {
    !state.is_menu_state()
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameSystemSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PauseMenuSystemSet;
