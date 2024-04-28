use std::io::{stdout, Write};

use bevy::prelude::*;
use serde::Deserialize;

// Plugin
pub struct TypingPlugin;
impl Plugin for TypingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TypingState>()
            .add_systems(Update, (read_input, handle_completed.after(read_input)));
    }
}

#[derive(Default, Deserialize, Asset, Debug, TypePath)]
pub struct Wordlists {
    deutsch: Vec<String>,
    english: Vec<String>,
}

#[derive(Debug, Clone, Reflect)]
pub enum Action {
    Test,
}

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct TypingState {
    available: Vec<(String, Action)>,
    current: Vec<(String, Action)>,
    progress: usize,
    completed: Vec<Action>,
}

fn read_input(mut chars: EventReader<ReceivedCharacter>, mut typing_state: ResMut<TypingState>) {
    // For each character typed
    chars.read().for_each(|event| {
        // Get the actual character
        let character = event.char.chars().next().unwrap();

        match typing_state.current.is_empty() {
            // If there are no words currently being typed
            true => {
                // Reset progress
                typing_state.progress = 0;

                // Add all words that start with the character to current
                let to_transfer: Vec<_> = typing_state
                    .available
                    .iter()
                    .filter(|&(word, _)| {
                        // If the strings first character
                        word.chars().next().unwrap() == character
                    })
                    .cloned()
                    .collect();
                typing_state.current.extend(to_transfer);
            }
            // If there are words currently being typed
            false => {
                // Increase progress
                typing_state.progress += 1;

                // Filter out words with non-matching characters, add completed words to completed
                // and then filter them out
                typing_state.current = typing_state
                    .current
                    .clone()
                    .into_iter()
                    .filter(|(word, action)| {
                        if word.chars().nth(typing_state.progress).unwrap() != character {
                            false
                        } else if typing_state.progress >= word.len() {
                            typing_state.completed.push(action.clone());
                            false
                        } else {
                            true
                        }
                    })
                    .collect();
            }
        }
    });
}

fn handle_completed(typing_state: Res<TypingState>) {
    for action in &typing_state.completed {
        match action {
            Action::Test => {}
        }
    }
}
