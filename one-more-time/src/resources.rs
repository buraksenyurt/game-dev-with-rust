use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub balance: f32,
    pub cook_donut_count: u8,
}
