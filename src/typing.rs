use bevy::prelude::*;
use serde::Deserialize;

use crate::asset_loader::Handles;

// Plugin
pub struct TypingPlugin;
impl Plugin for TypingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, read_input);
    }
}

#[derive(Default, Deserialize, Asset, Debug, TypePath)]
pub struct Wordlists {
    deutsch: Vec<String>,
    english: Vec<String>,
}

pub struct TypingState {
    available: Vec<String>,
    current: Vec<String>,
    progress: u8,
    completed: Vec<String>,
}

fn read_input(
    mut chars: EventReader<ReceivedCharacter>,
    wordlists: Res<Assets<Wordlists>>,
    handles: Res<Handles>,
) {
    chars.read().map(|event| event.char)
}
