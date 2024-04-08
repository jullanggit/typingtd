use bevy::prelude::*;

// Plugin
pub struct TypingPlugin;
impl Plugin for TypingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, typing);
    }
}

fn typing(keyboard_input: Res<ButtonInput<KeyCode>>) {}
