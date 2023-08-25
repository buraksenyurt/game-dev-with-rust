use crate::station::resources::*;
use crate::station::systems::*;
use bevy::app::App;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;
pub const FUEL_STATION_SPAWN_TIME: f32 = 30.;
pub struct FuelStationPlugin;
impl Plugin for FuelStationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FuelStationSpawnTimer>()
            .add_systems(Startup, spawn_fuel_station)
            .add_systems(
                Update,
                (
                    move_fuel_station,
                    check_outside_of_the_bounds,
                    count_fuel_station_spawn_tick,
                    spawn_after_time_finished,
                ),
            );
    }
}
