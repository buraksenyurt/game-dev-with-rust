use crate::player::systems::*;
use bevy::prelude::*;

pub mod components;
mod systems;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_player);
    }
}
