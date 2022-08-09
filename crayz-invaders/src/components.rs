use bevy::prelude::Component;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
