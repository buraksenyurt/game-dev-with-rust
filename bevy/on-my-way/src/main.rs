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
        .add_event::<GameOverEvent>()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugins((GamePlugin, MainMenuPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            (
                change_to_game_state,
                change_to_main_menu,
                handle_game_over,
                exit_game,
            ),
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
