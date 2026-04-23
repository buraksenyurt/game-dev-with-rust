use bevy::prelude::*;

use crate::game::ui::game_over_menu::components::*;
use crate::game::ui::game_over_menu::styles::*;

pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_game_over_menu(&mut commands, &asset_server);
}

pub fn build_game_over_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let game_over_menu_entity = commands
        .spawn((
            game_over_menu_node(),
            ZIndex(2),
            GameOverMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    game_over_menu_container_node(),
                    UiImage::new(asset_server.load("sprites/game_over_background.png")),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Game Over"),
                        get_title_text_font(asset_server),
                        TextColor(DEFAULT_TEXT_COLOR),
                    ));

                    parent.spawn((
                        Text::new(""),
                        get_title_text_font(asset_server),
                        TextColor(DEFAULT_TEXT_COLOR),
                    ));

                    parent.spawn((
                        Text::new(""),
                        get_title_text_font(asset_server),
                        TextColor(DEFAULT_TEXT_COLOR),
                    ));

                    parent.spawn((
                        Text::new("Your Score :"),
                        get_final_score_text_font(asset_server),
                        TextColor(DEFAULT_TEXT_COLOR),
                        FinalScoreText {},
                    ));

                    parent
                        .spawn((
                            Button,
                            button_node(),
                            BackgroundColor(DEFAULT_BUTTON_COLOR),
                            RestartButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Restart (F5)"),
                                get_button_text_font(asset_server),
                                TextColor(DEFAULT_TEXT_COLOR),
                            ));
                        });

                    parent
                        .spawn((
                            Button,
                            button_node(),
                            BackgroundColor(DEFAULT_BUTTON_COLOR),
                            MainMenuButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Main Menu (F2)"),
                                get_button_text_font(asset_server),
                                TextColor(DEFAULT_TEXT_COLOR),
                            ));
                        });

                    parent
                        .spawn((
                            Button,
                            button_node(),
                            BackgroundColor(DEFAULT_BUTTON_COLOR),
                            QuitButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Quit (ESC)"),
                                get_button_text_font(asset_server),
                                TextColor(DEFAULT_TEXT_COLOR),
                            ));
                        });
                });
        })
        .id();

    game_over_menu_entity
}

pub fn despawn_game_over_menu(mut commands: Commands, query: Query<Entity, With<GameOverMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Game Over",
                                get_title_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "",
                                get_title_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "",
                                get_title_text_style(asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Your Score :",
                                    get_final_score_text_style(asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        FinalScoreText {},
                    ));

                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: BackgroundColor::from(
                                    Color::hex(DEFAULT_BUTTON_COLOR).unwrap(),
                                ),
                                ..default()
                            },
                            RestartButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Restart (F5)",
                                        get_button_text_style(asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: BackgroundColor::from(
                                    Color::hex(DEFAULT_BUTTON_COLOR).unwrap(),
                                ),
                                ..default()
                            },
                            MainMenuButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Main Menu (F2)",
                                        get_button_text_style(asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: BackgroundColor::from(
                                    Color::hex(DEFAULT_BUTTON_COLOR).unwrap(),
                                ),
                                ..default()
                            },
                            QuitButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Quit (ESC)",
                                        get_button_text_style(asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        })
        .id();

    game_over_menu_entity
}

pub fn despawn_game_over_menu(mut commands: Commands, query: Query<Entity, With<GameOverMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
