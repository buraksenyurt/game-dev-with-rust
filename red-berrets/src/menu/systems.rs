use crate::global::{Difficulty, GameState, Level};
use crate::menu::{
    MenuButtonAction, MenuState, OnMainMenuScreen, OnSettingsDifficultyMenuScreen,
    OnSettingsMenuScreen, SelectedOption,
};
use bevy::app::AppExit;
use bevy::prelude::*;

pub fn menu_setup_system(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

pub fn main_menu_setup_system(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(250.),
        height: Val::Px(65.),
        margin: UiRect::all(Val::Px(20.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 32.,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: BackgroundColor::from(Color::RED),
                                ..default()
                            },
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Start Mission",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: BackgroundColor::from(Color::RED),
                                ..default()
                            },
                            MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: BackgroundColor::from(Color::RED),
                                ..default()
                            },
                            MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Credits",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: BackgroundColor::from(Color::RED),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Quit", button_text_style));
                        });
                });
        });
}

pub fn menu_action_system(
    query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
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
                BackgroundColor(Color::BLACK)
            }
            (Interaction::Hovered, Some(_)) => BackgroundColor(Color::ORANGE_RED),
            (Interaction::Hovered, None) => BackgroundColor(Color::BLUE),
            (Interaction::None, None) => BackgroundColor(Color::RED),
        }
    }
}

pub fn settings_menu_setup_system(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(200.),
        height: Val::Px(65.),
        margin: UiRect::all(Val::Px(20.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 32.,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    for (action, text) in [
                        (MenuButtonAction::SettingsControls, "Controls"),
                        (MenuButtonAction::SettingsDifficulty, "Difficulty"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ] {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: BackgroundColor(Color::RED),
                                    ..default()
                                },
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    text,
                                    button_text_style.clone(),
                                ));
                            });
                    }
                });
        });
}

pub fn difficulty_settings_menu_setup_system(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(200.),
        height: Val::Px(65.),
        margin: UiRect::all(Val::Px(20.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 32.,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnSettingsDifficultyMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    for (level, text) in [
                        (Level::Training, "Training"),
                        (Level::Normal, "Normal"),
                        (Level::Brutal, "Brutal"),
                    ] {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: BackgroundColor(Color::RED),
                                    ..default()
                                },
                                Difficulty(level),
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    text,
                                    button_text_style.clone(),
                                ));
                            });
                    }

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: BackgroundColor(Color::RED),
                                ..default()
                            },
                            MenuButtonAction::BackToSettings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Back", button_text_style));
                        });
                });
        });
}
