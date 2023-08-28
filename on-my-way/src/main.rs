mod events;
mod game;
mod main_menu;
mod systems;

use crate::events::GameOverEvent;
use crate::game::GamePlugin;
use crate::main_menu::MainMenuPlugin;
use crate::systems::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((GamePlugin, MainMenuPlugin))
        .add_event::<GameOverEvent>()
        .add_systems(Startup, (spawn_camera, spawn_stars, handle_game_over))
        .run();
}
