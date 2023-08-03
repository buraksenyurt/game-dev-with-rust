use crate::enums::{DonutType, Region};
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
    pub donut_type: DonutType,
    pub is_get: bool,
    pub direction: Region,
}

#[derive(Component, Debug, Clone)]
pub struct Donut {
    pub life_time: Timer,
    pub donut_type: DonutType,
    pub is_delivered: bool,
    pub is_leaved: bool,
    pub location: Vec3,
    pub penalty_cost: f32,
}

#[derive(Component)]
pub struct Desk {
    pub region: Region,
    pub donut_type: Option<DonutType>,
}
