use super::*;
use bevy::prelude::*;
#[derive(Resource)]
pub struct GameState {
    pub current_meteor_count: u8,
    pub spaceship_fuel_level: f32,
}
impl Default for GameState {
    fn default() -> Self {
        Self {
            current_meteor_count: 0,
            spaceship_fuel_level: DEFAULT_FUEL_LEVEL,
        }
    }
}
