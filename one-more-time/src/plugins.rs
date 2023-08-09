use crate::enums::GameState;
use crate::systems::{sys_change_visibility, sys_show_menu, sys_start_game};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (sys_start_game).run_if(in_state(GameState::MainMenu)),
        );
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sys_show_menu)
            .add_systems(Update, sys_change_visibility);
    }
}
