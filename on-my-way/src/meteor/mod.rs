use crate::meteor::resources::MeteorSpawnTimer;
use crate::meteor::systems::*;
use bevy::app::App;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;
pub const METEOR_SPAWN_TIME: f32 = 5.;
pub const MAX_METEOR_COUNT: u8 = 5;
pub const METEOR_ROTATE_DEGREE: f32 = 30.;
pub struct MeteorPlugin;
impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MeteorSpawnTimer>()
            .add_systems(Startup, spawn_meteor_system)
            .add_systems(
                Update,
                (
                    meteor_movement_system,
                    meteor_outside_of_the_bounds_system,
                    meteor_spawn_tick_counter_system,
                    meteor_spawn_after_timer_system,
                ),
            );
    }
}
