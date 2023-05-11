mod constants;
mod shapes;
mod systems;

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::systems::animation::animation;
use crate::systems::movement::movement;
use crate::systems::setup::setup_system;
use bevy::app::App;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::DefaultPlugins;
use bevy_rapier2d::plugin::NoUserData;
use bevy_rapier2d::prelude::{RapierDebugRenderPlugin, RapierPhysicsPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::NAVY))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Jumper - A Series Jumping Game :P".to_string(),
                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_startup_system(setup_system)
        .add_system(movement)
        .add_system(animation)
        .run();
}
