use crate::graphics::events::GraphicsWaitEvent;
use crate::graphics::systems::*;
use bevy::prelude::*;

pub mod events;
pub mod resources;
mod systems;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GraphicsWaitEvent>()
            .add_systems(Startup, load_assets)
            .add_systems(Update, (spawn_part, spawn_tiles, update_part_position));
    }
}
