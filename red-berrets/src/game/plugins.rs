use crate::game::components::OnPlayingScreen;
use crate::game::game_systems::{game_setup_system, playing_system};
use crate::global::GameState;
use crate::utility::despawn_screen;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Play), game_setup_system)
            .add_systems(Update, playing_system.run_if(in_state(GameState::Play)))
            .add_systems(OnExit(GameState::Play), despawn_screen::<OnPlayingScreen>);
    }
}
