use crate::game::station::resources::*;
use crate::game::station::systems::*;
use bevy::app::App;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;
pub const FUEL_STATION_SPAWN_TIME: f32 = 25.;
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
