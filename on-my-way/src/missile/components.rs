use bevy::prelude::*;
#[derive(Component, Clone)]
pub struct Missile {
    pub direction: Vec2,
    pub speed: f32,
    pub width: f32,
    pub fuel_cost: f32,
}
