mod components;
mod constant;
mod player;
mod resources;

use crate::constant::*;
use crate::player::PlayerPlugin;
use crate::resources::{GameTextures, WinSize};
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Crayz Invaders".to_string(),
            width: 480.0,
            height: 640.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(init_system)
        .run();
}

fn init_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let main_window = windows.get_primary_mut().unwrap();
    main_window.set_position(IVec2::new(0, 0));

    let (w_width, w_height) = (main_window.width(), main_window.height());
    let window_size = WinSize {
        width: w_width,
        height: w_height,
    };
    commands.insert_resource(window_size);

    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
    };
    commands.insert_resource(game_textures);
}
