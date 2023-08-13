use crate::global::{Difficulty, GameState};
use bevy::prelude::*;

pub fn game_setup_system(mut _commands: Commands, difficulty: Res<Difficulty>) {
    info!("Güncel zorluk seviyesi, {:?}", difficulty.0);
}

pub fn playing_system(time: Res<Time>, mut _game_state: ResMut<NextState<GameState>>) {}
