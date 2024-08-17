use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component, Debug)]
pub struct Dir(pub Vec2);

#[derive(Component)]
pub struct Tower;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct TowerBundle {
    pub tower: Tower,
    pub position: Position,
}

impl TowerBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            tower: Tower,
            position: Position(Vec2::new(x, y)),
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub direction: Dir,
    pub position: Position,
    pub velocity: Velocity,
}

impl PlayerBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            player: Player,
            direction: Dir(Vec2::new(1., 0.)),
            position: Position(Vec2::new(x, y)),
            velocity: Velocity(Vec2::ZERO),
        }
    }
}
