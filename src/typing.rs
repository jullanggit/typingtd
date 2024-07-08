use std::fmt::Display;

use bevy::{
    color::palettes::css::GREEN,
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use bevy_device_lang::get_lang;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use strum::EnumIter;

use crate::{
    asset_loader::Handles,
    physics::Layer,
    projectile::SpawnArrow,
    states::{change_menu_state, ChangeMenuState, GameState, MenuState, RunGame},
    tower::{ChangeTowerPriority, TowerPriority},
    upgrades::{ArrowTowerUpgrade, UpgradeTower},
};

// Plugin
pub struct TypingPlugin;
impl Plugin for TypingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Language>()
            .register_type::<ToType>()
            .add_systems(Startup, set_device_language)
            .add_systems(
                Update,
                (
                    read_input,
                    handle_text_display.after(read_input),
                    handle_to_types.after(read_input),
                ),
            )
            .observe(add_to_type)
            .observe(change_language)
            .observe(change_menu_state);
    }
}

#[derive(Resource, Debug, Clone, Copy, Reflect, Default, EnumIter)]
#[reflect(Resource)]
// pub struct Language(Languages);
// #[derive(Debug, Clone, Reflect, Default)]
pub enum Language {
    #[default]
    English,
    German,
}

#[derive(Default, Deserialize, Asset, Debug, TypePath)]
pub struct Wordlists {
    german: Vec<String>,
    english: Vec<String>,
}
impl Wordlists {
    /// Returns a random word from the inputted Language's wordlist
    pub fn get_word(&self, language: Language) -> String {
        match language {
            Language::English => {
                self.english[thread_rng().gen_range(0..self.english.len())].clone()
            }
            Language::German => self.german[thread_rng().gen_range(0..self.german.len())].clone(),
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub enum Action {
    SpawnArrow(Entity),
    ChangeLanguage(Language),
    ChangeMenuState(MenuState),
    RunGame,
    ChangeTowerPriority(Entity, TowerPriority),
    UpgradeTower(Entity, ArrowTowerUpgrade),
}
impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::SpawnArrow(_) => String::from("Shoot Arrow"),
                Self::ChangeLanguage(ref language) => format!("{language:?}"),
                Self::ChangeMenuState(ref menu) => format!("{menu}"),
                Self::RunGame => String::from("Run Game"),
                Self::ChangeTowerPriority(_, priority) => format!("{priority:?}"),
                Self::UpgradeTower(_, upgrade) => format!("{upgrade}"),
            }
        )
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ToType {
    pub word: String,
    pub progress: usize,
    pub action: Action,
}
impl ToType {
    pub const fn new(word: String, action: Action) -> Self {
        Self {
            word,
            progress: 0,
            action,
        }
    }
}

/// Sets the language based on the device language
fn set_device_language(mut language: ResMut<Language>) {
    let Some(language_string) = get_lang() else {
        return;
    };
    if language_string.contains("de") {
        *language = Language::German;
    }
}

/// Handles the input for the `ToTypes`
#[expect(clippy::wildcard_enum_match_arm)]
fn read_input(mut chars: EventReader<KeyboardInput>, mut to_types: Query<&mut ToType>) {
    // For each character typed
    chars
        .read()
        .filter(|&event| event.state == ButtonState::Pressed)
        .for_each(|event| {
            // Get the actual character
            let character = match event.logical_key {
                Key::Character(ref character) => character
                    .chars()
                    .next()
                    .expect("Character should exist if there is an event for it"),
                Key::Space => ' ',
                _ => {
                    return;
                }
            };

            to_types
                .iter_mut()
                // Filter out inactive to_types
                .for_each(|mut to_type| {
                    // If the typed character is the next character of the word
                    if to_type.word.chars().nth(to_type.progress) == Some(character) {
                        to_type.progress += 1;
                    // Otherwise reset the progress
                    } else {
                        to_type.progress = 0;
                    }
                });
        });
}

/// Executes the actions of any completed `ToTypes`, despawns them afterwards
fn handle_to_types(query: Query<(&ToType, &Parent, Entity)>, mut commands: Commands) {
    for (to_type, parent, entity) in &query {
        if to_type.progress >= to_type.word.chars().count() {
            handle_action(to_type.action.clone(), &mut commands);

            // Despawn entity
            commands.entity(parent.get()).remove_children(&[entity]);
            commands.entity(entity).despawn();
        }
    }
}

pub fn handle_action(action: Action, commands: &mut Commands<'_, '_>) {
    match action {
        Action::SpawnArrow(tower) => commands.trigger_targets(SpawnArrow, tower),
        Action::ChangeLanguage(language) => commands.trigger(ChangeLanguage(language)),
        Action::RunGame => commands.trigger(RunGame),
        Action::ChangeMenuState(state) => commands.trigger(ChangeMenuState(state)),
        Action::ChangeTowerPriority(tower, priority) => {
            commands.trigger_targets(ChangeTowerPriority(priority), tower);
        }
        Action::UpgradeTower(tower, upgrade) => {
            commands.trigger_targets(UpgradeTower(upgrade), tower);
        }
    }
}

#[derive(Debug, Clone, Event)]
pub struct ChangeLanguage(Language);

pub fn change_language(trigger: Trigger<ChangeLanguage>, mut language: ResMut<Language>) {
    *language = trigger.event().0;
}

#[derive(Debug, Clone, Event)]
pub struct AddToType(pub Action, pub Option<String>);

pub fn add_to_type(
    trigger: Trigger<AddToType>,
    mut commands: Commands,
    wordlists: Res<Assets<Wordlists>>,
    handles: Res<Handles>,
    language: Res<Language>,
    game_state: Res<State<GameState>>,
) {
    let AddToType(ref action, ref option_word) = *trigger.event();

    let word = match *option_word {
        Some(ref word) => word.clone(),
        None => wordlists
            .get(&handles.wordlists)
            .expect("Wordlists should be loaded")
            .get_word(*language)
            .replace("ß", "ss"),
    };

    commands.entity(trigger.entity()).with_children(|parent| {
        let mut entity = parent.spawn((
            Name::new("Text"),
            ToType::new(word.clone(), action.clone()),
            Layer::new(3.),
        ));

        // If the to type is going to be in a menu, use a diferent bundle
        if option_word.is_some() {
            entity.insert(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: String::new(),
                            style: TextStyle {
                                font: handles.font.clone(),
                                font_size: 25.,
                                color: Color::Srgba(GREEN),
                            },
                        },
                        TextSection {
                            value: word.clone(),
                            style: TextStyle {
                                font: handles.font.clone(),
                                font_size: 25.,
                                color: Color::WHITE,
                            },
                        },
                    ],
                    ..default()
                },
                ..default()
            });
        } else {
            entity.insert(Text2dBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: String::new(),
                            style: TextStyle {
                                font: handles.font.clone(),
                                font_size: 20.,
                                color: Color::Srgba(GREEN),
                            },
                        },
                        TextSection {
                            value: word.clone(),
                            style: TextStyle {
                                font: handles.font.clone(),
                                font_size: 20.,
                                color: Color::WHITE,
                            },
                        },
                    ],
                    ..default()
                },
                ..default()
            });
        }
        if *game_state.get() != GameState::Menu {
            entity.insert(StateScoped(*game_state.get()));
        }
    });
}

/// Changes character color based on word completion
fn handle_text_display(mut query: Query<(&ToType, &mut Text), Changed<ToType>>) {
    for (to_type, mut text) in &mut query {
        text.sections[0].value = to_type.word.chars().take(to_type.progress).collect();

        text.sections[1].value = to_type.word.chars().skip(to_type.progress).collect();
    }
}
