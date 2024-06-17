use bevy::prelude::*;
use bevy_device_lang::get_lang;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use strum::EnumIter;

use crate::{
    asset_loader::Handles,
    oneshot::OneShotSystems,
    physics::Position,
    projectile::{Speed, PROJECTILE_SPEED},
    upgrades::ArrowTowerUpgrades,
};

// Plugin
pub struct TypingPlugin;
impl Plugin for TypingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Language>()
            .add_systems(Startup, set_language)
            .add_systems(
                Update,
                (
                    read_input,
                    handle_text_display.after(read_input),
                    handle_actions.after(read_input),
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
    ShootArrow(Position, ArrowTowerUpgrades),
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

/// Sets the lanugage based on the device language
fn set_language(mut lanugage: ResMut<Language>) {
    let Some(language_string) = get_lang() else {
        return;
    };
    if language_string.contains("de") {
        *lanugage = Language::German;
    }
}

fn read_input(mut chars: EventReader<ReceivedCharacter>, mut to_types: Query<&mut ToType>) {
    // For each character typed
    chars.read().for_each(|event| {
        // Get the actual character
        let character = event
            .char
            .chars()
            .next()
            .expect("Character should exist if there is an event for it");

        for mut to_type in &mut to_types {
            if to_type.word.chars().nth(to_type.progress) == Some(character) {
                to_type.progress += 1;
            } else {
                to_type.progress = 0;
            }
        }
    });
}
fn handle_actions(
    query: Query<(&ToType, &Parent, Entity)>,
    mut commands: Commands,
    oneshot_systems: Res<OneShotSystems>,
) {
    for (to_type, parent, entity) in &query {
        if to_type.progress >= to_type.word.chars().count() {
            match to_type.action {
                Action::ShootArrow(position) => commands.run_system_with_input(
                    oneshot_systems.spawn_arrow,
                    (position, Speed::new(PROJECTILE_SPEED), upgrades),
                ),
            }
            commands.entity(parent.get()).remove_children(&[entity]);
            commands.entity(entity).despawn();
        }
    }
}

pub fn add_to_type(
    In((entity, action)): In<(Entity, Action)>,
    mut commands: Commands,
    wordlists: Res<Assets<Wordlists>>,
    handles: Res<Handles>,
    language: Res<Language>,
) {
    let word = wordlists
        .get(handles.wordlists.clone())
        .expect("Wordlists should be loaded")
        .get_word(&language)
        .replace("ß", "ss");

    commands.entity(entity).with_children(|parent| {
        parent.spawn((
            Name::new("Text"),
            Text2dBundle {
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
                            value: word.clone(),
                            style: TextStyle {
                                font: handles.font.clone(),
                                font_size: 20.,
                                color: Color::rgb_u8(255, 255, 255),
                            },
                        },
                    ],
                    ..default()
                },
                ..default()
            },
            ToType::new(word, action),
        ));
    });
}

fn handle_text_display(mut query: Query<(&ToType, &mut Text)>) {
    for (to_type, mut text) in &mut query {
        if text.sections[0].value.chars().count() != to_type.progress {
            text.sections[0].value = to_type.word.chars().take(to_type.progress).collect();

            text.sections[1].value = to_type.word.chars().skip(to_type.progress).collect();
        }
    }
}
