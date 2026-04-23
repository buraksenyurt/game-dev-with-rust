use crate::main_menu::components::{MainMenu, PlayButton, QuitButton};
use crate::main_menu::styles::*;
use bevy::color::palettes::css;
use bevy::prelude::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    if let Ok(entity) = query.single() {
        // despawn now recursively removes children too
        commands.entity(entity).despawn();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            MAIN_MENU_STYLE,
            BackgroundColor(Color::from(css::ORANGE_RED)),
            MainMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(TITLE_STYLE)
                .with_children(|parent| {
                    parent.spawn((
                        IMAGE_STYLE,
                        ImageNode::new(asset_server.load("sprites/ball_red_large.png")),
                    ));

                    parent.spawn((Text::new("Baller Game"), get_title_text_style(asset_server)));

                    parent.spawn((
                        IMAGE_STYLE,
                        ImageNode::new(asset_server.load("sprites/ball_blue_large.png")),
                    ));
                });

            parent
                .spawn((
                    Button,
                    BUTTON_STYLE,
                    BackgroundColor::from(DEFAULT_BUTTON_COLOR),
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play (Press G)"),
                        get_button_text_style(asset_server),
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    BUTTON_STYLE,
                    BackgroundColor::from(DEFAULT_BUTTON_COLOR),
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Quit Game (Press Esc)"),
                        get_button_text_style(asset_server),
                    ));
                });
        })
        .id();
    main_menu_entity
}
