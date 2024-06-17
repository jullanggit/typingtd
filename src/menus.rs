use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    states::{GameState, LanguageMenuSystemSet, MainMenuSystemSet},
    typing::Language,
};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_menu.in_set(MainMenuSystemSet));
        app.add_systems(
            Update,
            (
                check_input,
                pause_menu_button_interactions.in_set(LanguageMenuSystemSet),
                main_menu_button_interactions.in_set(MainMenuSystemSet),
            ),
        );
    }
}

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
struct Menu;

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
#[repr(transparent)]
pub struct LanguageButton {
    language: Language,
}
impl LanguageButton {
    pub const fn new(language: Language) -> Self {
        Self { language }
    }
}

fn check_input(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    menus: Query<Entity, With<Menu>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::LanguageMenu => {
                for menu in &menus {
                    commands.entity(menu).despawn_recursive();
                    next_state.set(GameState::Running);
                }
            }
            GameState::Running => {
                spawn_pause_menu(commands);
                next_state.set(GameState::LanguageMenu);
            }
            _ => {}
        }
    }
}

fn spawn_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgba(0.5, 0.5, 0.5, 0.6).into(),
                ..default()
            },
            Menu,
        ))
        .with_children(|parent: &mut ChildBuilder| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Languages",
                        TextStyle {
                            font_size: 80.0,
                            color: Color::DARK_GRAY,
                            ..default()
                        },
                    )],
                    ..default()
                },
                ..default()
            });
            for language in Language::iter() {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(200.0),
                                height: Val::Px(80.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::rgba(0.2, 0.2, 0.2, 0.8).into(),
                            ..default()
                        },
                        LanguageButton::new(language.clone()),
                    ))
                    .with_children(|parent: &mut ChildBuilder| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    format!("{language:?}"),
                                    TextStyle {
                                        font_size: 40.0,
                                        ..default()
                                    },
                                )],
                                ..default()
                            },
                            ..default()
                        });
                    });
            }
        });
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::rgba(0.5, 0.5, 0.5, 1.0).into(),
            ..default()
        },
        Menu,
    ));
}

// Menu interactions
const NORMAL_COLOR: Color = Color::rgba(0.2, 0.2, 0.2, 0.8);
const HOVERED_COLOR: Color = Color::rgba(0.2, 0.2, 0.2, 0.4);
const PRESSED_COLOR: Color = Color::rgba(0.2, 0.2, 0.2, 1.0);

pub fn pause_menu_button_interactions(
    mut buttons: Query<(&Interaction, &LanguageButton, &mut BackgroundColor), Changed<Interaction>>,
    mut language: ResMut<Language>,
) {
    for (interaction, language_button, mut background_color) in &mut buttons {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_COLOR.into();
                *language = language_button.language.clone();
            }
            Interaction::Hovered => *background_color = HOVERED_COLOR.into(),
            Interaction::None => *background_color = NORMAL_COLOR.into(),
        }
    }
}

pub fn main_menu_button_interactions() {}
