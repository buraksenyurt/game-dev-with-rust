use crate::missile::systems::*;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub struct MissilePlugin;

impl Plugin for MissilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_missile, check_outside_of_the_bounds));
    }
}
