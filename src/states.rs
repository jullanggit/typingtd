use std::fmt::Display;

use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    oneshot::OneShotSystems,
    tower::TowerPriority,
    typing::{Action, Language},
    upgrades::ArrowTowerUpgrade,
};

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .configure_sets(Update, GameSystemSet.run_if(not_in_menu_state))
            .configure_sets(Update, PauseMenuSystemSet.run_if(in_menu_state));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameSystemSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PauseMenuSystemSet;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum GameState {
    #[default]
    Loading,
    Running,
    // Menus
    MainMenu,
    PauseMenu,
    LanguageMenu,
    TowerSelectionMenu,
    SelectedTower(Entity),
    TowerUpgradeMenu(Entity),
    TowerPriorityMenu(Entity),
}
impl GameState {
    /// If the State is a menu state, returns a the actions for the buttons in the menu
    pub fn get_buttons(&self) -> Option<Vec<Action>> {
        match self {
            Self::Loading | Self::Running => None,
            Self::MainMenu => Some(vec![Action::ChangeState(Self::Running)]),
            Self::PauseMenu => Some(
                [Self::LanguageMenu, Self::TowerSelectionMenu]
                    .into_iter()
                    .map(Action::ChangeState)
                    .collect(),
            ),
            Self::LanguageMenu => Some(Language::iter().map(Action::ChangeLanguage).collect()),
            Self::TowerSelectionMenu => Some(Vec::new()),
            Self::SelectedTower(entity) => Some(
                [
                    Self::TowerUpgradeMenu(*entity),
                    Self::TowerPriorityMenu(*entity),
                ]
                .into_iter()
                .map(Action::ChangeState)
                .collect(),
            ),
            Self::TowerUpgradeMenu(entity) => Some(
                ArrowTowerUpgrade::iter()
                    .map(|upgrade| Action::UpgradeTower(*entity, upgrade))
                    .collect(),
            ),
            Self::TowerPriorityMenu(entity) => Some(
                TowerPriority::iter()
                    .map(|priority| Action::ChangeTowerPriority(*entity, priority))
                    .collect(),
            ),
        }
    }
    pub const fn is_menu_state(&self) -> bool {
        match self {
            Self::Running | Self::Loading => false,
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
                Self::Loading => "Loading",
                Self::Running => "Play",
                Self::MainMenu => "Main Menu",
                Self::PauseMenu => "Options",
                Self::LanguageMenu => "Languages",
                Self::TowerSelectionMenu => "Select Tower",
                Self::SelectedTower(_) => "Select Option",
                Self::TowerUpgradeMenu(_) => "Upgrades",
                Self::TowerPriorityMenu(_) => "Priorities",
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

/// Changes the state to the given state, spawns and despawns menus if necessary
pub fn change_state(
    In(state_to_set): In<GameState>,
    mut commands: Commands,
    oneshot_systems: Res<OneShotSystems>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if current_state.get().is_menu_state() {
        // Despawn all menus and set game state to running
        commands.run_system(oneshot_systems.despawn_menus);
    }
    if state_to_set.is_menu_state() {
        // Spawn Pause menu and set game state to pause menu
        commands.run_system_with_input(oneshot_systems.spawn_menu, state_to_set.clone());
    }
    next_state.set(state_to_set);
}
