mod constants;

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use bevy::app::App;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Jumper - A Series Jumping Game :P".to_string(),
                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_startup_system(setup_system)
        .run();
}

fn setup_system() {
    println!("2D Platform Game");
}
