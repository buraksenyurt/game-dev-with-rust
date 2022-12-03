use crate::entity::enemy_type::EnemyType;
use macroquad::prelude::Vec2;

pub struct Enemy {
    pub location: Vec2,
    pub velocity: Vec2,
    pub direction: Vec2,
    pub is_alive: bool,
    pub enemy_type: EnemyType,
}
