use crate::board::resources::ActiveBoard;
use crate::board::systems::spawn_board_map;
use crate::states::AppState;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveBoard>()
            .add_systems(OnEnter(AppState::Game), spawn_board_map);
    }
}
