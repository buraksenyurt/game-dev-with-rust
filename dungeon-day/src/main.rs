use crate::action::ActionsPlugin;
use crate::asset::AssetPlugin;
use crate::board::BoardPlugin;
use crate::globals::*;
use crate::graphics::GraphicsPlugin;
use crate::input::InputPlugin;
use crate::manager::ManagerPlugin;
use crate::parts::PartsPlugin;
use crate::player::PlayerPlugin;
use crate::states::{AppState, GameState};
use crate::systems::*;
use bevy::prelude::*;

mod action;
mod asset;
mod board;
mod globals;
mod graphics;
mod input;
mod manager;
mod parts;
mod player;
mod states;
mod systems;
mod utility;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WIN_WIDTH, WIN_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(Msaa::Off)
        .add_state::<AppState>()
        .add_state::<GameState>()
        .add_plugins((
            AssetPlugin,
            BoardPlugin,
            GraphicsPlugin,
            PlayerPlugin,
            InputPlugin,
            ActionsPlugin,
            ManagerPlugin,
            PartsPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .run()
}
