use bevy::{color::palettes::css::DARK_GRAY, prelude::*, sprite::Anchor};

use crate::{
    asset_loader::Handles,
    enemy::{Health, Life, Money},
    states::{ChangeMenuState, GameState, MenuState, PauseMenuSystemSet, RunGame},
    tower::Tower,
    typing::{Action, AddToType, handle_action},
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
            .add_observer(spawn_menu);
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
    life_text: Query<Entity, (With<LifeText>, Changed<Health>)>,
    mut writer: TextUiWriter,
    life: Res<Life>,
) {
    if let Ok(life_text) = life_text.get_single() {
        *writer.text(life_text, 0) = format!("{} Lives", life.value);
    }
}

fn spawn_life_display(mut commands: Commands, handles: Res<Handles>) {
    commands.spawn((
        Name::new("Life display"),
        Text2d::new(String::new()),
        TextFont {
            font: handles.font.clone(),
            font_size: 80.0,
            ..default()
        },
        TextColor::BLACK,
        // Previously:
        // style: Style {
        //     position_type: PositionType::Absolute,
        //     left: Val::Px(10.),
        //     top: Val::Px(10.),
        // },
        Anchor::TopLeft,
        LifeText,
    ));
}

fn update_money_text(
    money_text: Query<Entity, With<MoneyText>>,
    mut writer: TextUiWriter,
    money: Res<Money>,
) {
    if let Ok(money_text) = money_text.get_single() {
        *writer.text(money_text, 0) = format!("{}$", money.value);
    }
}

fn spawn_money_text(mut commands: Commands, handles: Res<Handles>) {
    commands.spawn((
        Name::new("Money display"),
        Text2d::new(String::new()),
        TextFont {
            font: handles.font.clone(),
            font_size: 80.0,
            ..default()
        },
        TextColor::BLACK,
        // Previously:
        // style: Style {
        //     position_type: PositionType::Absolute,
        //     right: Val::Px(10.),
        //     top: Val::Px(10.),
        // },
        Anchor::TopRight,
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
        Sprite {
            image: handles.menu_image.clone(),
            custom_size: Some(Vec2::new(1024., 576.)),
            ..default()
        },
        StateScoped(MenuState::MainMenu),
    ));
}

#[derive(Debug, Clone, Event)]
pub struct SpawnMenu(pub MenuState);

pub fn spawn_menu(trigger: Trigger<SpawnMenu>, mut commands: Commands) {
    let menu = trigger.event().0;
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.6)),
            Menu,
            StateScoped(menu),
        ))
        .with_children(|parent: &mut ChildBuilder| {
            parent.spawn((
                Text2d::new(format!("{menu}")),
                TextFont::from_font_size(80.),
                TextColor(Color::Srgba(DARK_GRAY)),
            ));
            for button_action in menu.get_buttons() {
                parent.spawn((
                    Button,
                    Node {
                        width: Val::Px(15. + button_action.to_string().len() as f32 * 22.),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
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
