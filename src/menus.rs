use crate::menu_interaction::*;
use bevy::prelude::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuOpen>().add_systems(
            Update,
            (
                check_input,
                interact_with_english_button,
                interact_with_german_button,
            ),
        );
    }
}

#[derive(Resource, Debug, Clone, Reflect, Default)]
#[reflect(Resource)]
struct MenuOpen {
    open: bool,
}

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
struct Menu;

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct EnglishButton;

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct GermanButton;

fn check_input(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    menu_entity: Query<Entity, With<Menu>>,
    mut menu_open: ResMut<MenuOpen>,
) {
    if input.just_pressed(KeyCode::Escape) && !menu_open.open {
        spawn_menu(commands);
        menu_open.open = true;
    } else if input.just_pressed(KeyCode::Escape) && menu_open.open {
        for entity in &menu_entity {
            commands.entity(entity).despawn_recursive();
            menu_open.open = false;
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
                    EnglishButton,
                ))
                .with_children(|parent: &mut ChildBuilder| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "English",
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
                    GermanButton,
                ))
                .with_children(|parent: &mut ChildBuilder| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Deutsch",
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
        });
}