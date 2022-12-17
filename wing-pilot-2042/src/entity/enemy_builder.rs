use crate::common::constants::ENEMY_ENTRY_POINT_FACTOR;
use crate::entity::enemy::Enemy;
use crate::entity::enemy_type::EnemyType::Warship;
use crate::entity::enemy_type::{EnemyType, WarshipDirection};
use crate::entity::formation::get_formation;
use macroquad::math::Vec2;
use macroquad::prelude::{rand, screen_height, screen_width};

pub async fn create_enemies(count: usize, e_type: EnemyType) -> Vec<Enemy> {
    let x = rand::gen_range(screen_width() * 0.2, screen_width() - screen_width() * 0.2);
    let mut y = 0.;
    let mut position = Vec2::new(x, -ENEMY_ENTRY_POINT_FACTOR);

    match e_type {
        Warship(wd) => match wd {
            WarshipDirection::Left => {
                y = rand::gen_range(
                    screen_height() * 0.3,
                    screen_height() - screen_height() * 0.3,
                );
                position = Vec2::new(-ENEMY_ENTRY_POINT_FACTOR * 3., y);
            }
            WarshipDirection::Right => {
                y = rand::gen_range(
                    screen_height() * 0.3,
                    screen_height() - screen_height() * 0.3,
                );
                position = Vec2::new(screen_width() + ENEMY_ENTRY_POINT_FACTOR * 3., y);
            }
        },
        _ => {}
    }

    let mut enemies: Vec<Enemy> = Vec::new();
    let formation = get_formation().await;
    for i in 0..count {
        let mut enemy = Enemy::new(position, e_type, formation).await;
        match e_type {
            Warship(_) => {}
            _ => enemy.position.y -= i as f32 * (enemy.texture.height() * 1.5),
        };
        enemies.push(enemy);
    }
    enemies
}
