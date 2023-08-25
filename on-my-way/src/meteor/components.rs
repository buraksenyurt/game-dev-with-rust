use bevy::prelude::*;
#[derive(Component)]
pub struct Meteor {
    pub direction: Vec2,
    pub speed: f32,
    pub width: f32,
}
