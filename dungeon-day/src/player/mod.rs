use crate::player::systems::spawn_player;
use crate::states::AppState;
use bevy::prelude::*;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player);
    }
}
