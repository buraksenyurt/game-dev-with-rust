use crate::asset::AssetPlugin;
use crate::globals::{WIN_HEIGHT, WIN_WIDTH};
use crate::states::AppState;
use bevy::prelude::*;

mod asset;
mod globals;
mod states;

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
        .add_plugins(AssetPlugin)
        .run()
}
