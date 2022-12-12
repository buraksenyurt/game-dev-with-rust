use crate::common::constants::FLEET_LIFT_OFF_TIME;
use crate::entity::bullet::Bullet;
use crate::entity::enemy::Enemy;
use crate::entity::enemy_builder::create_enemies;
use crate::entity::enemy_type::EnemyType;
use macroquad::prelude::rand;
use macroquad::time::get_frame_time;

pub struct Fleet {
    pub actors: Vec<Enemy>,
    pub lift_off_time: i32,
    pub bullets: Vec<Bullet>,
}

impl Fleet {
    pub async fn new(count: usize, e_type: EnemyType) -> Self {
        Self {
            actors: create_enemies(count, e_type).await,
            lift_off_time: rand::gen_range(
                get_frame_time() as i32,
                get_frame_time() as i32 + FLEET_LIFT_OFF_TIME,
            ),
            bullets: Vec::default(),
        }
    }
}

impl Default for Fleet {
    fn default() -> Self {
        Self {
            actors: Vec::default(),
            lift_off_time: rand::gen_range(
                get_frame_time() as i32,
                get_frame_time() as i32 + FLEET_LIFT_OFF_TIME,
            ),
            bullets: Vec::default(),
        }
    }
}
