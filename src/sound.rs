use crate::asset_loader::Handles;
use bevy::prelude::*;

pub struct SoundPlugin;
impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, handles: Res<Handles>) {
    commands.spawn((
        Name::new("Background music"),
        AudioPlayer(handles.background_music.clone()),
        PlaybackSettings::LOOP,
    ));
}
