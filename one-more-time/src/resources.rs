use crate::components::Customer;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub balance: f32,
    pub cook_donut_count: u8,
    pub customers_inside: Vec<Customer>,
}
