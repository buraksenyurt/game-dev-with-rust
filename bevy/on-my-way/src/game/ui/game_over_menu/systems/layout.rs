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
                    ImageNode::new(asset_server.load("sprites/game_over_background.png")),
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
    if let Ok(entity) = query.single() {
        commands.entity(entity).despawn();
    }
}

