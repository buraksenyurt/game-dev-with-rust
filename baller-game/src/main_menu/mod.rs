mod components;
mod styles;
mod systems;

use crate::main_menu::systems::interactions::*;
use crate::main_menu::systems::layout::*;
use crate::AppState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                (pressed_play_game_button, pressed_quit_game_button)
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
        //.add_systems(Startup, main_menu);
    }
}
