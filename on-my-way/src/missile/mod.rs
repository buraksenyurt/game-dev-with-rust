use crate::missile::systems::*;
use bevy::prelude::*;

pub mod components;
mod systems;
pub struct MissilePlugin;

impl Plugin for MissilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_missile,
                detect_collision_with_meteors,
                claim_hitted,
                check_outside_of_the_bounds,
            ),
        );
    }
}
