use crate::main_menu::components::{MainMenu, PlayButton, QuitButton};
use crate::main_menu::styles::*;
use bevy::prelude::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    if let Ok(entity) = query.single() {
        commands.entity(entity).despawn();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            main_menu_node(),
            ZIndex(2),
            MainMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    main_menu_container_node(),
                    ImageNode::new(asset_server.load("sprites/main_background.png")),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Mars Yolcusu"),
                        get_title_text_font(asset_server),
                        TextColor(DEFAULT_TEXT_COLOR),
                    ));

                    parent
                        .spawn((
                            Button,
                            button_node(),
                            BackgroundColor(DEFAULT_BUTTON_COLOR),
                            PlayButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Play (Press F5)"),
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
                                Text::new("Quit Game (Press Esc)"),
                                get_button_text_font(asset_server),
                                TextColor(DEFAULT_TEXT_COLOR),
                            ));
                        });
                });
        })
        .id();
    main_menu_entity
}
