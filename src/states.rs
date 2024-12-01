use std::fmt::Display;

use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    menus::SpawnMenu,
    tower::TowerPriority,
    typing::{Action, Language},
    upgrades::ArrowTowerUpgrade,
};

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<MenuState>()
            .configure_sets(Update, GameSystemSet.run_if(in_state(GameState::Running)))
            .configure_sets(Update, PauseMenuSystemSet.run_if(in_state(GameState::Menu)))
            .enable_state_scoped_entities::<GameState>()
            .enable_state_scoped_entities::<MenuState>()
            .add_observer(run_game)
            .add_observer(change_menu_state);
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameSystemSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PauseMenuSystemSet;

#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[source(GameState = GameState::Menu)]
pub enum MenuState {
    #[default]
    MainMenu,
    PauseMenu,
    LanguageMenu,
    TowerSelectionMenu,
    SelectedTower(Entity),
    TowerUpgradeMenu(Entity),
    TowerPriorityMenu(Entity),
}
impl MenuState {
    pub fn get_buttons(&self) -> Vec<Action> {
        match *self {
            Self::MainMenu => vec![Action::RunGame],
            Self::PauseMenu => [Self::LanguageMenu, Self::TowerSelectionMenu]
                .into_iter()
                .map(Action::ChangeMenuState)
                .collect(),
            Self::LanguageMenu => Language::iter().map(Action::ChangeLanguage).collect(),
            Self::TowerSelectionMenu => Vec::new(),
            Self::SelectedTower(entity) => [
                Self::TowerUpgradeMenu(entity),
                Self::TowerPriorityMenu(entity),
            ]
            .into_iter()
            .map(Action::ChangeMenuState)
            .collect(),
            Self::TowerUpgradeMenu(entity) => ArrowTowerUpgrade::iter()
                .map(|upgrade| Action::UpgradeTower(entity, upgrade))
                .collect(),
            Self::TowerPriorityMenu(entity) => TowerPriority::iter()
                .map(|priority| Action::ChangeTowerPriority(entity, priority))
                .collect(),
        }
    }
}
impl Display for MenuState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Self::MainMenu => "Main Menu",
            Self::PauseMenu => "Options",
            Self::LanguageMenu => "Languages",
            Self::TowerSelectionMenu => "Select Tower",
            Self::SelectedTower(_) => "Select Option",
            Self::TowerUpgradeMenu(_) => "Upgrades",
            Self::TowerPriorityMenu(_) => "Priorities",
        })
    }
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum GameState {
    #[default]
    Loading,
    Running,
    Menu,
}

#[derive(Debug, Clone, Event)]
pub struct RunGame;

pub fn run_game(_trigger: Trigger<RunGame>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Running);
}

#[derive(Debug, Clone, Event)]
pub struct ChangeMenuState(pub MenuState);

pub fn change_menu_state(
    trigger: Trigger<ChangeMenuState>,
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    let state_to_set = trigger.event().0;
    next_game_state.set(GameState::Menu);
    next_menu_state.set(state_to_set);
    commands.trigger(SpawnMenu(state_to_set));
}
