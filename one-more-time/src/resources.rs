use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub gold_value: i32,
    pub cook_donut_count: u8,
}
