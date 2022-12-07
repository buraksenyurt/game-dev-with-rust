use crate::common::constants::ENEMY_ENTRY_POINT_FACTOR;
use crate::entity::enemy::Enemy;
use crate::entity::enemy_type::EnemyType;
use macroquad::math::Vec2;
use macroquad::prelude::{rand, screen_width};

pub async fn create_enemies(count: usize, e_type: EnemyType) -> Vec<Enemy> {
    let x = rand::gen_range(screen_width() * 0.2, screen_width() - screen_width() * 0.2);
    let entry_point = Vec2::new(x, -ENEMY_ENTRY_POINT_FACTOR);
    let mut enemies: Vec<Enemy> = Vec::new();
    for i in 0..count {
        let mut enemy = Enemy::new(entry_point, e_type).await;
        enemy.velocity = Vec2::new(0., 1.);
        enemy.location.y -= i as f32 * enemy.texture.height() + 20.;
        enemies.push(enemy);
    }
    enemies
}
