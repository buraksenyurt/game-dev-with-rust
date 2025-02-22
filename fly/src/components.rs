use bevy::math::Vec3;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Box;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Resource)]
pub struct BoxSpawningTimer(pub Timer);
