use crate::game::station::FUEL_STATION_SPAWN_TIME;
use bevy::prelude::*;

#[derive(Resource)]
pub struct FuelStationSpawnTimer {
    pub timer: Timer,
}
impl Default for FuelStationSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(FUEL_STATION_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
