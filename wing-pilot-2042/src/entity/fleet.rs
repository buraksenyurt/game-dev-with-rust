use crate::common::constants::FLEET_LIFT_OFF_TIME;
use crate::entity::enemy::Enemy;
use crate::entity::enemy_builder::create_enemies;
use crate::entity::enemy_type::EnemyType;
use macroquad::prelude::{get_fps, rand};

pub struct Fleet {
    pub enemies: Vec<Enemy>,
    pub lift_off_time: i32,
}

impl Fleet {
    pub async fn new(count: usize, e_type: EnemyType) -> Self {
        Self {
            enemies: create_enemies(count, e_type).await,
            lift_off_time: rand::gen_range(
                get_fps() as i32,
                get_fps() as i32 + FLEET_LIFT_OFF_TIME,
            ),
        }
    }
}

impl Default for Fleet {
    fn default() -> Self {
        Self {
            enemies: Vec::default(),
            lift_off_time: rand::gen_range(
                get_fps() as i32,
                get_fps() as i32 + FLEET_LIFT_OFF_TIME,
            ),
        }
    }
}
