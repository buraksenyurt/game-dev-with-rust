use crate::spaceship::resources::*;
use crate::spaceship::systems::*;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub const SPACESHIP_001_WIDTH: f32 = 80.;
pub const SPACESHIP_001_HEIGHT: f32 = 106.;
pub const SPACESHIP_001_SPEED: f32 = 500.;
pub const FUEL_DECREASE_PERIOD: f32 = 5.;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FuelCheckTimer>()
            .add_systems(Startup, spawn_spaceship_system)
            .add_systems(
                Update,
                (
                    spaceship_movement_system,
                    spaceship_border_check_system,
                    fuel_tick_counter_system,
                    meteors_spaceship_collision_detection_system,
                    decrease_spaceship_fuel_system,
                ),
            );
    }
}
