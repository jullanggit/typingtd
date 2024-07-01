use crate::asset_loader::Handles;
use bevy::{audio::PlaybackMode, prelude::*};

pub struct SoundPlugin;
impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, handles: Res<Handles>) {
    commands.spawn((
        Name::new("Background music"),
        AudioBundle {
            source: handles.background_music.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
    ));
}
