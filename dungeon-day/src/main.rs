use crate::asset::AssetPlugin;
use crate::board::BoardPlugin;
use crate::globals::{WIN_HEIGHT, WIN_WIDTH};
use crate::states::AppState;
use bevy::prelude::*;

mod asset;
mod board;
mod globals;
mod states;
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
        .add_plugins((AssetPlugin, BoardPlugin))
        .run()
}
