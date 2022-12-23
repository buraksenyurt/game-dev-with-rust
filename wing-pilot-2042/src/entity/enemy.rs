use crate::common::constants::*;
use crate::entity::bullet::Bullet;
use crate::entity::enemy_type::{EnemyType, WarshipDirection};
use crate::entity::formation::Formation;
use crate::entity::owner::Owner;
use macroquad::color::WHITE;
use macroquad::prelude::{
    draw_texture, get_frame_time, load_texture, screen_height, screen_width, Rect, Texture2D, Vec2,
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
    pub shield: i32,
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
                Some(WarshipDirection::Right) => (
                    load_texture("resources/warship_right.png").await.unwrap(),
                    Vec2::new(-1., 0.),
                ),
                Some(WarshipDirection::Left) => (
                    load_texture("resources/warship_left.png").await.unwrap(),
                    Vec2::new(1., 0.),
                ),
                _ => (
                    load_texture("resources/warship_left.png").await.unwrap(),
                    Vec2::new(1., 0.),
                ),
            },
        };
        let shield = match enemy_type {
            EnemyType::Fighter => ENEMY_FIGHTER_SHIELD,
            EnemyType::Bomber => ENEMY_BOMBER_SHIELD,
            EnemyType::Warship(Some(_ws)) => ENEMY_WARSHIP_SHIELD,
            _ => 0,
        };
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
            shield,
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

    pub async fn get_muzzle_point(&self) -> Vec2 {
        match self.enemy_type {
            EnemyType::Warship(_) => Vec2::new(
                self.position.x + self.texture.width() * 0.8,
                self.position.y + self.texture.height() * 0.5,
            ),
            _ => Vec2::new(
                self.position.x + self.texture.width() * 0.5,
                self.position.y + self.texture.height(),
            ),
        }
    }

    pub async fn get_body(&self) -> Option<Rect> {
        match self.enemy_type {
            EnemyType::Fighter => Some(Rect::new(
                self.position.x + self.texture.width() * 0.5 - 15.,
                self.position.y,
                30.,
                self.texture.height(),
            )),
            EnemyType::Bomber => Some(Rect::new(
                self.position.x + self.texture.width() * 0.5 - 2.5,
                self.position.y,
                5.,
                self.texture.height(),
            )),
            EnemyType::Warship(Some(_w)) => Some(Rect::new(
                self.position.x,
                self.position.y,
                self.texture.width(),
                self.texture.height(),
            )),
            _ => None,
        }
    }
    pub async fn get_wing(&self) -> Option<Rect> {
        match self.enemy_type {
            EnemyType::Fighter => Some(Rect::new(
                self.position.x,
                self.position.y + self.texture.height() * 0.6,
                self.texture.width(),
                20.,
            )),
            EnemyType::Bomber => Some(Rect::new(
                self.position.x,
                self.position.y + self.texture.height() * 0.6,
                self.texture.width(),
                20.,
            )),
            _ => None,
        }
    }

    pub async fn get_tail_wing(&self) -> Rect {
        Rect::new(self.position.x + 40., self.position.y, 40., 10.)
    }

    pub async fn spawn_bullets(&mut self, vel: Vec2, rad: f32) -> Option<Vec<Bullet>> {
        if self.cooling <= 0. {
            match self.enemy_type {
                EnemyType::Bomber => {
                    let cm = Vec2::new(
                        self.position.x + self.texture.width() * 0.5,
                        self.position.y + self.texture.height(),
                    );
                    let mut bullet = Bullet::spawn(Owner::EnemyBomber, cm).await;
                    bullet.velocity = vel;
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
                    bullet_1.velocity = vel;
                    let mut bullet_2 = Bullet::spawn(Owner::EnemyFighter, rm).await;
                    bullet_2.velocity = vel;
                    self.cooling = get_frame_time() * get_fps() as f32 * 0.35;

                    Some(vec![bullet_1, bullet_2])
                }
                EnemyType::Warship(_) => {
                    let dm = Vec2::new(
                        self.position.x + self.texture.width() * 0.8,
                        self.position.y + self.texture.height() * 0.5,
                    );
                    let mut bullet_1 = Bullet::spawn(Owner::Warship, dm).await;
                    bullet_1.velocity = vel;
                    bullet_1.rotation = rad;
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
