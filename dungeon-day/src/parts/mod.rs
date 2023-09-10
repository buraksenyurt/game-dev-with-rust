use crate::parts::systems::spawn_non_player_characters;
use crate::states::AppState;
use bevy::app::App;
use bevy::prelude::{OnEnter, Plugin};

pub mod components;
mod systems;

pub struct PartsPlugin;

impl Plugin for PartsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_non_player_characters);
    }
}
