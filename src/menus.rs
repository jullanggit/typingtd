use bevy::prelude::*;

use crate::{
    asset_loader::Handles,
    enemy::{Life, Money},
    oneshot::OneShotSystems,
    states::{GameState, PauseMenuSystemSet},
    tower::Tower,
    typing::{handle_action, Action},
};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MenuButton>()
            .add_systems(Startup, spawn_menu_image)
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(
                OnExit(GameState::MainMenu),
                (spawn_money_text, spawn_life_display),
            )
            .add_systems(
                OnEnter(GameState::TowerSelectionMenu),
                add_tower_selection_to_types,
            )
            .add_systems(
                Update,
                (
                    toggle_pause_menu,
                    button_interactions.in_set(PauseMenuSystemSet),
                    add_menu_button_to_type.in_set(PauseMenuSystemSet),
                    update_money_text.run_if(resource_changed::<Money>),
                ),
            );
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

pub fn update_life_text(life_text: &mut Query<&mut Text, With<LifeText>>, life: f64) {
    if let Ok(mut life_text) = life_text.get_single_mut() {
        life_text.sections[0].value = format!("{} Lives", life);
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

fn spawn_main_menu(mut commands: Commands, oneshot_systems: Res<OneShotSystems>) {
    commands.run_system_with_input(oneshot_systems.spawn_menu, GameState::MainMenu);
}

fn toggle_pause_menu(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    oneshot_systems: Res<OneShotSystems>,
    current_state: Res<State<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        // Toggles to_types
        commands.run_system(oneshot_systems.toggle_to_type);

        if current_state.get().is_menu_state() {
            commands.run_system_with_input(oneshot_systems.change_state, GameState::Running);
            commands.run_system(oneshot_systems.remove_inactive_to_types);
        } else {
            commands.run_system_with_input(oneshot_systems.change_state, GameState::PauseMenu);
        }
    }
}

pub fn despawn_menus(mut commands: Commands, menus: Query<Entity, With<Menu>>) {
    for menu in &menus {
        commands.entity(menu).despawn_recursive();
    }
}

fn spawn_menu_image(asset_server: Res<AssetServer>, mut commands: Commands) {
    let texture_handle: Handle<Image> = asset_server.load("menu_image.png");

    commands.spawn((
        Name::new("menu image"),
        Menu,
        SpriteBundle {
            texture: texture_handle,
            sprite: Sprite {
                custom_size: Some(Vec2::new(1024., 576.)),
                ..default()
            },
            ..default()
        },
    ));
}

pub fn spawn_menu(In(menu): In<GameState>, mut commands: Commands) {
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
                        format!("{menu}"),
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
            for button_action in menu
                .get_buttons()
                .expect("Provided game state should be a menu state")
            {
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(15. + button_action.to_string().len() as f32 * 22.),
                            height: Val::Px(80.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgba(0.2, 0.2, 0.2, 0.8).into(),
                        ..default()
                    },
                    MenuButton::new(button_action),
                ));
            }
        });
}

fn add_tower_selection_to_types(
    towers: Query<Entity, With<Tower>>,
    mut commands: Commands,
    oneshot_systems: Res<OneShotSystems>,
) {
    for tower in &towers {
        commands.run_system_with_input(
            oneshot_systems.add_to_type,
            (
                tower,
                Action::ChangeState(GameState::SelectedTower(tower)),
                None,
            ),
        );
    }
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
