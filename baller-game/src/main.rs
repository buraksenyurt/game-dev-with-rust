mod common;
mod events;
mod game;
mod main_menu;
mod systems;

use bevy::prelude::*;

use crate::game::{GamePlugin, PlayGroundState};
use crate::main_menu::MainMenuPlugin;
use systems::*;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // My Plugins
        .add_plugins((MainMenuPlugin, GamePlugin))
        // Startup systems
        .add_systems(Startup, spawn_camera)
        // Update Systems
        .add_systems(
            Update,
            (
                exit_game,
                handle_game_over,
                change_to_game_state,
                change_to_main_menu,
            ),
        )
        .add_systems(
            Update,
            refresh_everything
                .run_if(in_state(AppState::Game))
                .run_if(in_state(PlayGroundState::Running)),
        )
        .run();
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
