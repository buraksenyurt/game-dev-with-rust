use bevy::prelude::*;
#[derive(Component)]
pub struct Missile {
    pub direction: Vec2,
    pub speed: f32,
    pub width: f32,
}
