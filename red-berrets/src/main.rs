mod game;
mod global;
mod menu;
mod splash;
mod utility;

use splash::*;

use crate::game::plugins::GamePlugin;
use crate::game::systems::*;
use crate::global::*;
use crate::menu::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<GameState>()
        .insert_resource(Difficulty(Level::Normal))
        .add_systems(Startup, (setup_system, soldier_setup_system))
        .add_systems(Update, soldier_animation_system)
        .add_plugins((SplashPlugin, MenuPlugin, GamePlugin))
        .run();
}
