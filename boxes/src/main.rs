mod board;
mod components;
mod globals;
mod player;
mod systems;

use crate::board::BoardPlugin;
use crate::globals::{WIN_HEIGHT, WIN_WIDTH};
use crate::player::PlayerPlugin;
use crate::systems::setup_camera;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WIN_WIDTH, WIN_HEIGHT).into(),
                        title: "Box Game".into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((BoardPlugin, PlayerPlugin))
        .add_systems(Startup, setup_camera)
        .run();
}
