use crate::common::constants::COOLING_FACTOR;
use crate::entity::bullet::Bullet;
use crate::entity::enemy_type::{EnemyType, WarshipDirection};
use crate::entity::formation::Formation;
use crate::entity::owner::Owner;
use macroquad::color::WHITE;
use macroquad::prelude::{
    draw_texture, get_frame_time, load_texture, screen_height, screen_width, Texture2D, Vec2,
};
use macroquad::time::get_fps;

pub struct Enemy {
    pub position: Vec2,
    pub velocity: Vec2,
    pub formation: Formation,
    pub is_alive: bool,
    pub enemy_type: EnemyType,
    pub texture: Texture2D,
    pub is_formation_on: bool,
    pub fire_at_will: bool,
    pub on_stage: bool,
    cooling: f32,
}

impl Enemy {
    pub async fn new(position: Vec2, enemy_type: EnemyType, formation: Formation) -> Self {
        let (texture, velocity) = match enemy_type {
            EnemyType::Bomber => (
                load_texture("resources/bomber.png").await.unwrap(),
                Vec2::new(0., 1.),
            ),
            EnemyType::Fighter => (
                load_texture("resources/enemy_fighter.png").await.unwrap(),
                Vec2::new(0., 1.),
            ),
            EnemyType::Warship(wd) => match wd {
                WarshipDirection::Right => (
                    load_texture("resources/warship_right.png").await.unwrap(),
                    Vec2::new(-1., 0.),
                ),
                WarshipDirection::Left => (
                    load_texture("resources/warship_left.png").await.unwrap(),
                    Vec2::new(1., 0.),
                ),
            },
        };
        //let formation = get_formation();
        Self {
            position,
            enemy_type,
            is_alive: true,
            velocity,
            formation,
            texture,
            is_formation_on: false,
            fire_at_will: false,
            on_stage: true,
            cooling: get_frame_time(),
        }
    }
    pub async fn draw(&self) {
        draw_texture(self.texture, self.position.x, self.position.y, WHITE);
    }

    pub async fn check_borders(&mut self) {
        if (self.velocity.y < 0. && self.position.y + self.texture.height() < 0.)
            || (self.velocity.x < 0. && self.position.x + self.texture.width() < 0.)
            || (self.position.x > screen_width() + self.texture.width()
                || self.position.y > screen_height() + self.texture.height())
        {
            self.on_stage = false;
        }
    }

    pub fn get_muzzle_point(&self) -> Vec2 {
        Vec2::new(
            self.position.x + self.texture.width() * 0.5,
            self.position.y + self.texture.height(),
        )
    }
    pub async fn spawn_bullets(&mut self, bullet_velocity: Vec2) -> Option<Vec<Bullet>> {
        if self.cooling <= 0. {
            match self.enemy_type {
                EnemyType::Bomber => {
                    let cm = Vec2::new(
                        self.position.x + self.texture.width() * 0.5,
                        self.position.y + self.texture.height(),
                    );
                    let mut bullet = Bullet::spawn(Owner::EnemyBomber, cm).await;
                    bullet.velocity = bullet_velocity;
                    self.cooling = get_frame_time() * get_fps() as f32 * 1.65;
                    Some(vec![bullet])
                }
                EnemyType::Fighter => {
                    let lm = Vec2::new(
                        self.position.x + self.texture.width() * 0.2,
                        self.position.y + self.texture.height() * 0.8,
                    );
                    let rm = Vec2::new(
                        self.position.x + (self.texture.width() - self.texture.width() * 0.2),
                        self.position.y + self.texture.height() * 0.8,
                    );
                    let mut bullet_1 = Bullet::spawn(Owner::EnemyFighter, lm).await;
                    bullet_1.velocity = bullet_velocity;
                    let mut bullet_2 = Bullet::spawn(Owner::EnemyFighter, rm).await;
                    bullet_2.velocity = bullet_velocity;
                    self.cooling = get_frame_time() * get_fps() as f32 * 0.35;

                    Some(vec![bullet_1, bullet_2])
                }
                EnemyType::Warship(_) => {
                    let dm = Vec2::new(
                        self.position.x + self.texture.width() * 0.8,
                        self.position.y + self.texture.height() * 0.5,
                    );
                    let mut bullet_1 = Bullet::spawn(Owner::Warship, dm).await;
                    bullet_1.velocity = bullet_velocity;
                    self.cooling = get_frame_time() * get_fps() as f32 * 4.;

                    Some(vec![bullet_1])
                }
            }
        } else {
            self.cooling -= get_frame_time() * COOLING_FACTOR;
            None
        }
    }
}
