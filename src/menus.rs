use bevy::{color::palettes::css::DARK_GRAY, prelude::*};

use crate::{
    asset_loader::Handles,
    enemy::{Health, Life, Money},
    states::{ChangeMenuState, GameState, MenuState, PauseMenuSystemSet, RunGame},
    tower::Tower,
    typing::{handle_action, Action, AddToType},
};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MenuButton>()
            .add_systems(OnEnter(MenuState::MainMenu), spawn_menu_image)
            .add_systems(OnEnter(MenuState::MainMenu), spawn_main_menu)
            .add_systems(
                OnExit(MenuState::MainMenu),
                (spawn_money_text, spawn_life_display),
            )
            .add_systems(
                OnEnter(MenuState::TowerSelectionMenu),
                add_tower_selection_to_types,
            )
            .add_systems(
                Update,
                (
                    toggle_pause_menu,
                    button_interactions.in_set(PauseMenuSystemSet),
                    add_menu_button_to_type.in_set(PauseMenuSystemSet),
                    update_money_text.run_if(resource_changed::<Money>),
                    update_life_text.run_if(resource_changed::<Life>),
                ),
            )
            .observe(spawn_menu);
    }
}

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct Menu;

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct MoneyText;

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct LifeText;

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

pub fn update_life_text(
    mut life_text: Query<&mut Text, (With<LifeText>, Changed<Health>)>,
    life: Res<Life>,
) {
    if let Ok(mut life_text) = life_text.get_single_mut() {
        life_text.sections[0].value = format!("{} Lives", life.value);
    }
}

fn spawn_life_display(mut commands: Commands, handles: Res<Handles>) {
    commands.spawn((
        Name::new("Life display"),
        TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    String::new(),
                    TextStyle {
                        font: handles.font.clone(),
                        font_size: 80.0,
                        color: Color::BLACK,
                    },
                )],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                top: Val::Px(10.),
                ..default()
            },
            ..default()
        },
        LifeText,
    ));
}

fn update_money_text(mut money_text: Query<&mut Text, With<MoneyText>>, money: Res<Money>) {
    if let Ok(mut money_text) = money_text.get_single_mut() {
        money_text.sections[0].value = format!("{}$", money.value);
    }
}

fn spawn_money_text(mut commands: Commands, handles: Res<Handles>) {
    commands.spawn((
        Name::new("Money display"),
        TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    String::new(),
                    TextStyle {
                        font: handles.font.clone(),
                        font_size: 80.0,
                        color: Color::BLACK,
                    },
                )],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(10.),
                top: Val::Px(10.),
                ..default()
            },
            ..default()
        },
        MoneyText,
    ));
}

fn spawn_main_menu(mut commands: Commands) {
    commands.trigger(SpawnMenu(MenuState::MainMenu));
}

fn toggle_pause_menu(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    game_state: Res<State<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        if *game_state.get() == GameState::Menu {
            commands.trigger(RunGame);
        } else {
            commands.trigger(ChangeMenuState(MenuState::PauseMenu));
        }
    }
}

fn spawn_menu_image(mut commands: Commands, handles: Res<Handles>) {
    commands.spawn((
        Name::new("menu image"),
        Menu,
        SpriteBundle {
            texture: handles.menu_image.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1024., 576.)),
                ..default()
            },
            ..default()
        },
    ));
}

#[derive(Debug, Clone, Event)]
pub struct SpawnMenu(pub MenuState);

pub fn spawn_menu(trigger: Trigger<SpawnMenu>, mut commands: Commands) {
    let menu = trigger.event().0;
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
                background_color: Color::srgba(0.5, 0.5, 0.5, 0.6).into(),
                ..default()
            },
            Menu,
        ))
        .with_children(|parent: &mut ChildBuilder| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        format!("{menu}"),
                        TextStyle {
                            font_size: 80.0,
                            color: Color::Srgba(DARK_GRAY),
                            ..default()
                        },
                    )],
                    ..default()
                },
                ..default()
            });
            for button_action in menu.get_buttons() {
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(15. + button_action.to_string().len() as f32 * 22.),
                            height: Val::Px(80.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgba(0.2, 0.2, 0.2, 0.8).into(),
                        ..default()
                    },
                    MenuButton::new(button_action),
                ));
            }
        });
}

fn add_tower_selection_to_types(towers: Query<Entity, With<Tower>>, mut commands: Commands) {
    for tower in &towers {
        commands.trigger_targets(
            AddToType(
                Action::ChangeMenuState(MenuState::SelectedTower(tower)),
                None,
            ),
            tower,
        );
    }
}

fn add_menu_button_to_type(
    menu_buttons: Query<(Entity, &MenuButton), Without<Children>>,
    mut commands: Commands,
) {
    for (entity, menu_button) in &menu_buttons {
        commands.trigger_targets(
            AddToType(
                menu_button.action.clone(),
                Some(format!("{}", menu_button.action)),
            ),
            entity,
        );
    }
}

// Menu interactions
const NORMAL_COLOR: Color = Color::srgba(0.2, 0.2, 0.2, 0.8);
const HOVERED_COLOR: Color = Color::srgba(0.2, 0.2, 0.2, 0.4);
const PRESSED_COLOR: Color = Color::srgba(0.2, 0.2, 0.2, 1.0);

pub fn button_interactions(
    mut buttons: Query<(&Interaction, &MenuButton, &mut BackgroundColor), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (interaction, menu_button, mut background_color) in &mut buttons {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_COLOR.into();

                handle_action(menu_button.action.clone(), &mut commands);
            }
            Interaction::Hovered => *background_color = HOVERED_COLOR.into(),
            Interaction::None => *background_color = NORMAL_COLOR.into(),
        }
    }
}
