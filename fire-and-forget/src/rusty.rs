use macroquad::prelude::{Color, Vec2};

pub struct Rusty {
    pub start_position: Vec2,
    pub current_position: Vec2,
    pub direction: Vec2,
    pub speed: Vec2,
    pub color: Color,
}
