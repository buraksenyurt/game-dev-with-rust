use crate::common::constants::COOLING_FACTOR;
use crate::entity::bullet::Bullet;
use crate::entity::enemy_type::EnemyType;
use crate::entity::formation::Formation;
use crate::entity::owner::Owner;
use macroquad::color::WHITE;
use macroquad::prelude::{draw_texture, get_frame_time, load_texture, Texture2D, Vec2};
use macroquad::time::get_fps;

pub struct Enemy {
    pub position: Vec2,
    pub velocity: Vec2,
    pub formation: Formation,
    pub is_alive: bool,
    pub enemy_type: EnemyType,
    pub texture: Texture2D,
    pub is_formation_on: bool,
    pub on_stage: bool,
    cooling: f32,
}

impl Enemy {
    pub async fn new(location: Vec2, enemy_type: EnemyType, formation: Formation) -> Self {
        let texture = match enemy_type {
            EnemyType::Bomber => load_texture("resources/bomber.png").await.unwrap(),
            EnemyType::Fighter => load_texture("resources/enemy_fighter.png").await.unwrap(),
            EnemyType::Warship => load_texture("resources/warship.png").await.unwrap(),
        };
        //let formation = get_formation();
        Self {
            position: location,
            enemy_type,
            is_alive: true,
            velocity: Vec2::default(),
            formation,
            texture,
            is_formation_on: false,
            on_stage: true,
            cooling: get_frame_time(),
        }
    }
    pub fn draw(&self) {
        draw_texture(self.texture, self.position.x, self.position.y, WHITE);
    }

    pub async fn spawn_bullets(&mut self) -> Option<Vec<Bullet>> {
        if self.cooling <= 0. {
            match self.enemy_type {
                EnemyType::Bomber => {
                    let cm = Vec2::new(
                        self.position.x + self.texture.width() * 0.5,
                        self.position.y + self.texture.height() * 0.5,
                    );
                    let bullet = Bullet::spawn(Owner::EnemyBomber, cm).await;
                    self.cooling = get_frame_time() * get_fps() as f32;

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
                    let bullet_1 = Bullet::spawn(Owner::EnemyFighter, lm).await;
                    let bullet_2 = Bullet::spawn(Owner::EnemyFighter, rm).await;
                    self.cooling = get_frame_time() * get_fps() as f32 * 1.25;

                    Some(vec![bullet_1, bullet_2])
                }
                EnemyType::Warship => {
                    let um = Vec2::new(
                        self.position.x + self.texture.width() * 0.5,
                        self.position.y + self.texture.height() * 0.2,
                    );
                    let dm = Vec2::new(
                        self.position.x + self.texture.width() * 0.5,
                        self.position.y + self.texture.height() * 0.8,
                    );
                    let bullet_1 = Bullet::spawn(Owner::Warship, um).await;
                    let bullet_2 = Bullet::spawn(Owner::Warship, dm).await;
                    self.cooling = get_frame_time() * get_fps() as f32;

                    Some(vec![bullet_1, bullet_2])
                }
            }
        } else {
            self.cooling -= get_frame_time() * COOLING_FACTOR;
            None
        }
    }
}
