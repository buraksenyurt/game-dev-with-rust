use bevy::prelude::*;
#[derive(Component)]
pub struct FuelStation {
    pub direction: Vec2,
    pub speed: f32,
    pub fuel_amount: f32,
}
