use crate::board::resources::{ActiveBoard, AssetList};
use crate::board::systems::*;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>()
            .init_resource::<ActiveBoard>()
            .add_systems(Startup, (load_assets, load_board))
            .add_systems(Update, (spawn_parts, update_part_position));
    }
}
