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
            .add_systems(Startup, spawn_spaceship)
            .add_systems(
                Update,
                (
                    check_outside_of_the_bounds,
                    count_fuel_tick,
                    detect_collision_with_meteors,
                    decrease_spaceship_fuel,
                    detect_connected_with_fuel_station,
                    move_spaceship,
                ),
            );
    }
}