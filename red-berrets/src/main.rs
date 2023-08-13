mod global;
mod menu;
mod splash;
mod utility;

use splash::*;

use crate::global::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_systems(Startup, setup_system)
        .add_plugins(SplashPlugin)
        .run();
}
