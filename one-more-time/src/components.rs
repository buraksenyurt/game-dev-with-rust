use crate::enums::DonutType;
use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct Customer {
    pub speed: f32,
}

#[derive(Component)]
pub struct Donut {
    pub life_time: Timer,
    pub donut_type: DonutType,
}
