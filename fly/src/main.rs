mod constants;
mod setup;

use crate::setup::setup_system;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_system)
        .run();
}
