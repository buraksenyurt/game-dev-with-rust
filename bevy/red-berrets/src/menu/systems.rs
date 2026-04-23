use crate::global::{Difficulty, GameState, Level};
use crate::menu::{
    MenuButtonAction, MenuState, OnMainMenuScreen, OnSettingsDifficultyMenuScreen,
    OnSettingsMenuScreen, SelectedOption,
};
use bevy::app::AppExit;
use bevy::color::palettes::css;
use bevy::prelude::*;

pub fn menu_setup_system(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

pub fn main_menu_setup_system(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(250.),
        height: Val::Px(65.),
        margin: UiRect::all(Val::Px(20.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::from(Srgba::new(0.86, 0.08, 0.24, 1.0))),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(Color::from(css::RED)),
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            parent.spawn(Text::new("Start Mission"));
                        });

                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(Color::from(css::RED)),
                            MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(Text::new("Settings"));
                        });

                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(Color::from(css::RED)),
                            MenuButtonAction::Credits,
                        ))
                        .with_children(|parent| {
                            parent.spawn(Text::new("Credits"));
                        });

                    parent
                        .spawn((
                            Button,
                            button_node,
                            BackgroundColor(Color::from(css::RED)),
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(Text::new("Quit"));
                        });
                });
        });
}

pub fn menu_action_system(
    query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: MessageWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Play);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::SettingsControls => {
                    menu_state.set(MenuState::SettingsControls);
                }
                MenuButtonAction::SettingsDifficulty => {
                    menu_state.set(MenuState::SettingsDifficulty);
                }
                MenuButtonAction::Credits => menu_state.set(MenuState::Credits),
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
            }
        }
    }
}

pub fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => {
                BackgroundColor(Color::from(css::BLACK))
            }
            (Interaction::Hovered, Some(_)) => BackgroundColor(Color::from(css::ORANGE_RED)),
            (Interaction::Hovered, None) => BackgroundColor(Color::from(css::BLUE)),
            (Interaction::None, None) => BackgroundColor(Color::from(css::RED)),
        };
    }
}

pub fn settings_menu_setup_system(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(200.),
        height: Val::Px(65.),
        margin: UiRect::all(Val::Px(20.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::from(Srgba::new(0.86, 0.08, 0.24, 1.0))),
                ))
                .with_children(|parent| {
                    for (action, text) in [
                        (MenuButtonAction::SettingsControls, "Controls"),
                        (MenuButtonAction::SettingsDifficulty, "Difficulty"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ] {
                        parent
                            .spawn((
                                Button,
                                button_node.clone(),
                                BackgroundColor(Color::from(css::RED)),
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn(Text::new(text));
                            });
                    }
                });
        });
}

pub fn difficulty_settings_menu_setup_system(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(200.),
        height: Val::Px(65.),
        margin: UiRect::all(Val::Px(20.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnSettingsDifficultyMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::from(Srgba::new(0.86, 0.08, 0.24, 1.0))),
                ))
                .with_children(|parent| {
                    for (level, text) in [
                        (Level::Training, "Training"),
                        (Level::Normal, "Normal"),
                        (Level::Brutal, "Brutal"),
                    ] {
                        parent
                            .spawn((
                                Button,
                                button_node.clone(),
                                BackgroundColor(Color::from(css::RED)),
                                Difficulty(level),
                            ))
                            .with_children(|parent| {
                                parent.spawn(Text::new(text));
                            });
                    }

                    parent
                        .spawn((
                            Button,
                            button_node,
                            BackgroundColor(Color::from(css::RED)),
                            MenuButtonAction::BackToSettings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(Text::new("Back"));
                        });
                });
        });
}
