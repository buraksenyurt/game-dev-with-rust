mod constant;
mod system;

use crate::constant::{CLEAR_COLOR, RESOLUTION};
use crate::system::camera::spawn_camera;
use crate::system::player::spawn_player;
use crate::system::texture::load_ascii;
use bevy::prelude::*;

fn main() {
    let height = 900.;
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Dragon Fighter".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .add_plugins(DefaultPlugins)
        .run();
}
