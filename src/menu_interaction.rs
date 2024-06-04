use bevy::prelude::*;

use crate::{menus::*, typing::Language};

const NORMAL_COLOR: Color = Color::rgba(0.2, 0.2, 0.2, 0.8);
const HOVERED_COLOR: Color = Color::rgba(0.2, 0.2, 0.2, 0.4);
const PRESSED_COLOR: Color = Color::rgba(0.2, 0.2, 0.2, 1.0);

pub fn interact_with_english_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<EnglishButton>),
    >,
    mut language: ResMut<Language>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_COLOR.into();
                *language = Language::English;
            }
            Interaction::Hovered => *background_color = HOVERED_COLOR.into(),
            Interaction::None => *background_color = NORMAL_COLOR.into(),
        }
    }
}

pub fn interact_with_german_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<GermanButton>),
    >,
    mut language: ResMut<Language>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_COLOR.into();
                *language = Language::German;
            }
            Interaction::Hovered => *background_color = HOVERED_COLOR.into(),
            Interaction::None => *background_color = NORMAL_COLOR.into(),
        }
    }
}
