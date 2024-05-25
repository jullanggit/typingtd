use bevy::prelude::*;
use rand::{thread_rng, Rng};
use serde::Deserialize;

// Plugin
pub struct TypingPlugin;
impl Plugin for TypingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Language>()
            .add_systems(Update, read_input);
    }
}

#[derive(Resource, Debug, Clone, Reflect, Default)]
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
    ShootArrow,
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
