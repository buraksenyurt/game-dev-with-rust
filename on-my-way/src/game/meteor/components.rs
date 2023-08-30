use bevy::prelude::*;
#[derive(Component)]
pub struct Meteor {
    pub direction: Vec2,
    pub speed: f32,
    pub width: f32,
    pub current_hit_count: u8,
    pub strength: u8,
}
