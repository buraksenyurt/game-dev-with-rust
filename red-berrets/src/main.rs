mod game;
mod global;
mod menu;
mod splash;
mod utility;

use splash::*;

use crate::game::plugins::GamePlugin;
use crate::global::*;
use crate::menu::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .insert_resource(Difficulty(Level::Normal))
        .add_systems(Startup, setup_system)
        .add_plugins((SplashPlugin, MenuPlugin, GamePlugin))
        .run();
}
