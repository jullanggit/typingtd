use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    oneshot::OneShotSystems,
    states::{GameState, LanguageMenuSystemSet},
    typing::{handle_action, Action, Language},
};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MenuButton>().add_systems(
            Update,
            (
                toggle_pause_menu,
                button_interactions.in_set(LanguageMenuSystemSet),
                add_menu_button_to_type.in_set(LanguageMenuSystemSet),
            ),
        );
    }
}

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
struct Menu;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
#[repr(transparent)]
pub struct MenuButton {
    action: Action,
}
impl MenuButton {
    pub const fn new(action: Action) -> Self {
        Self { action }
    }
}

fn toggle_pause_menu(
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
                spawn_menu(commands);
                next_state.set(GameState::LanguageMenu);
            }
        }
    }
}

fn spawn_menu(mut commands: Commands) {
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
                parent.spawn((
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
                    MenuButton::new(Action::ChangeLanguage(language.clone())),
                ));
            }
        });
}

fn add_menu_button_to_type(
    menu_buttons: Query<(Entity, &MenuButton), Without<Children>>,
    mut commands: Commands,
    oneshot_systems: Res<OneShotSystems>,
) {
    for (entity, menu_button) in &menu_buttons {
        commands.run_system_with_input(
            oneshot_systems.add_to_type,
            (
                entity,
                menu_button.action.clone(),
                Some(format!("{}", menu_button.action)),
            ),
        );
    }
}

// Menu interactions
const NORMAL_COLOR: Color = Color::rgba(0.2, 0.2, 0.2, 0.8);
const HOVERED_COLOR: Color = Color::rgba(0.2, 0.2, 0.2, 0.4);
const PRESSED_COLOR: Color = Color::rgba(0.2, 0.2, 0.2, 1.0);

pub fn button_interactions(
    mut buttons: Query<(&Interaction, &MenuButton, &mut BackgroundColor), Changed<Interaction>>,
    mut commands: Commands,
    oneshot_systems: Res<OneShotSystems>,
) {
    for (interaction, menu_button, mut background_color) in &mut buttons {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_COLOR.into();

                handle_action(menu_button.action.clone(), &mut commands, &oneshot_systems);
            }
            Interaction::Hovered => *background_color = HOVERED_COLOR.into(),
            Interaction::None => *background_color = NORMAL_COLOR.into(),
        }
    }
}
