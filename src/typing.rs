use std::fmt::Display;

use bevy::prelude::*;
use bevy_device_lang::get_lang;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use strum::EnumIter;

use crate::{
    asset_loader::Handles,
    oneshot::OneShotSystems,
    physics::{Layer, Position},
    projectile::{Speed, PROJECTILE_SPEED},
    states::GameState,
    tower::TowerPriority,
    upgrades::{ArrowTowerUpgrade, ArrowTowerUpgrades},
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
            );
    }
}

#[derive(Resource, Debug, Clone, Reflect, Default, EnumIter)]
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
    pub fn get_word(&self, language: &Language) -> String {
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
    ShootArrow(Position, ArrowTowerUpgrades, TowerPriority),
    ChangeLanguage(Language),
    ChangeState(GameState),
    ChangeTowerPriority(Entity, TowerPriority),
    UpgradeTower(Entity, ArrowTowerUpgrade),
}
impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ShootArrow(_, _, _) => String::from("Shoot Arrow"),
                Self::ChangeLanguage(language) => format!("{language:?}"),
                Self::ChangeState(menu) => format!("{menu}"),
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
    pub active: bool,
}
impl ToType {
    pub const fn new(word: String, action: Action) -> Self {
        Self {
            word,
            progress: 0,
            action,
            active: true,
        }
    }
}

/// Sets the lanugage based on the device language
fn set_device_language(mut lanugage: ResMut<Language>) {
    let Some(language_string) = get_lang() else {
        return;
    };
    if language_string.contains("de") {
        *lanugage = Language::German;
    }
}

/// Handles the input for the `ToTypes`
fn read_input(mut chars: EventReader<ReceivedCharacter>, mut to_types: Query<&mut ToType>) {
    // For each character typed
    chars.read().for_each(|event| {
        // Get the actual character
        let character = event
            .char
            .chars()
            .next()
            .expect("Character should exist if there is an event for it");

        // If the character isnt the escape buttons character (so that going into the pause menu
        // doesnt reset the progress)
        if character != '\x1B' {
            to_types
                .iter_mut()
                // Filter out inactive to_types
                .filter(|to_type| to_type.active)
                .for_each(|mut to_type| {
                    // If the typed character is the next character of the word
                    if to_type.word.chars().nth(to_type.progress) == Some(character) {
                        to_type.progress += 1;
                    // Otherwise reset the progress
                    } else {
                        to_type.progress = 0;
                    }
                });
        }
    });
}

/// Executes the actions of any completed `ToTypes`, despawns them afterwards
fn handle_to_types(
    query: Query<(&ToType, &Parent, Entity)>,
    mut commands: Commands,
    oneshot_systems: Res<OneShotSystems>,
) {
    for (to_type, parent, entity) in &query {
        if to_type.progress >= to_type.word.chars().count() {
            handle_action(to_type.action.clone(), &mut commands, &oneshot_systems);

            // Despawn entity
            commands.entity(parent.get()).remove_children(&[entity]);
            commands.entity(entity).despawn();
        }
    }
}

pub fn handle_action(
    action: Action,
    commands: &mut Commands<'_, '_>,
    oneshot_systems: &Res<'_, OneShotSystems>,
) {
    match action {
        Action::ShootArrow(position, upgrades, priority) => commands.run_system_with_input(
            oneshot_systems.spawn_arrow,
            (position, Speed::new(PROJECTILE_SPEED), upgrades, priority),
        ),
        Action::ChangeLanguage(language) => {
            commands.run_system_with_input(oneshot_systems.change_language, language);
        }
        Action::ChangeState(state) => {
            commands.run_system_with_input(oneshot_systems.change_state, state);
        }
        Action::ChangeTowerPriority(tower, priority) => {
            commands
                .run_system_with_input(oneshot_systems.change_tower_priority, (tower, priority));
        }
        Action::UpgradeTower(tower, upgrade) => {
            commands.run_system_with_input(oneshot_systems.upgrade_tower, (tower, upgrade));
        }
    }
}

pub fn change_language(In(new_language): In<Language>, mut language: ResMut<Language>) {
    *language = new_language;
}

pub fn add_to_type(
    In((entity, action, option_word)): In<(Entity, Action, Option<String>)>,
    mut commands: Commands,
    wordlists: Res<Assets<Wordlists>>,
    handles: Res<Handles>,
    language: Res<Language>,
) {
    let word = match option_word {
        Some(ref word) => word.clone(),
        None => wordlists
            .get(handles.wordlists.clone())
            .expect("Wordlists should be loaded")
            .get_word(&language)
            .replace("ß", "ss"),
    };

    commands.entity(entity).with_children(|parent| {
        let mut entity = parent.spawn((
            Name::new("Text"),
            ToType::new(word.clone(), action),
            Layer::new(3.),
        ));

        if option_word.is_some() {
            entity.insert(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: String::new(),
                            style: TextStyle {
                                font: handles.font.clone(),
                                font_size: 25.,
                                color: Color::GREEN,
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
                                color: Color::GREEN,
                            },
                        },
                        TextSection {
                            value: word,
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
    });
}

/// Changes character color based on word completion
fn handle_text_display(mut query: Query<(&ToType, &mut Text), Changed<ToType>>) {
    for (to_type, mut text) in &mut query {
        text.sections[0].value = to_type.word.chars().take(to_type.progress).collect();

        text.sections[1].value = to_type.word.chars().skip(to_type.progress).collect();
    }
}

/// Toggles the "active" state of all `ToTypes`, hides them if inactive
pub fn toggle_to_type(mut to_types: Query<(&mut ToType, &mut Visibility)>) {
    for (mut to_type, mut visibility) in &mut to_types {
        to_type.active = !to_type.active;

        if to_type.active {
            *visibility = Visibility::Inherited;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn remove_inactive_to_types(
    to_types: Query<(Entity, &Parent, &ToType)>,
    mut commands: Commands,
) {
    for (entity, parent, to_type) in &to_types {
        if !to_type.active {
            commands.entity(parent.get()).remove_children(&[entity]);
            commands.entity(entity).despawn();
        }
    }
}
