use crate::common::constants::{COOLING_FACTOR, FIGHTER_SPEED_FACTOR, MAX_AMMO};
use crate::entity::bullet::Bullet;
use crate::entity::owner::Owner;
use macroquad::color::WHITE;
use macroquad::prelude::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D, Vec2};
use macroquad::time::get_frame_time;
use macroquad::window::{screen_height, screen_width};

pub struct Fighter {
    pub position: Vec2,
    pub life: usize,
    pub bullets: Vec<Bullet>,
    texture: Texture2D,
    pub ammo_count: usize,
    cooling: f32,
}

impl Fighter {
    pub async fn new() -> Self {
        let texture = load_texture("resources/fighter.png").await.unwrap();
        let position = Vec2::new(
            screen_width() * 0.5 - texture.width() * 0.5,
            screen_height() - texture.height(),
        );
        Self {
            position,
            life: 3,
            bullets: Vec::new(),
            texture,
            ammo_count: MAX_AMMO,
            cooling: get_frame_time(),
        }
    }

    pub async fn spawn_bullets(&mut self) -> Option<Vec<Bullet>> {
        if self.cooling <= 0. {
            let lm = Vec2::new(
                self.position.x + self.texture.width() * 0.2,
                self.position.y,
            );
            let rm = Vec2::new(
                self.position.x + (self.texture.width() - self.texture.width() * 0.2),
                self.position.y,
            );
            let bullet_1 = Bullet::spawn(Owner::Fighter, lm).await;
            let bullet_2 = Bullet::spawn(Owner::Fighter, rm).await;
            self.cooling = get_frame_time();
            Some(vec![bullet_1, bullet_2])
        } else {
            self.cooling -= get_frame_time() * COOLING_FACTOR;
            None
        }
    }

    pub async fn shift_left(&mut self) {
        if self.position.x <= 0. {
            return;
        }
        self.position -= Vec2::new(1., 0.) * FIGHTER_SPEED_FACTOR;
    }
    pub async fn shift_right(&mut self) {
        if self.position.x >= screen_width() - self.texture.width() {
            return;
        }
        self.position += Vec2::new(1., 0.) * FIGHTER_SPEED_FACTOR;
    }
    pub async fn shift_up(&mut self) {
        if self.position.y < 0. {
            return;
        }
        self.position -= Vec2::new(0., 1.) * FIGHTER_SPEED_FACTOR;
    }
    pub async fn shift_down(&mut self) {
        if self.position.y > screen_height() - self.texture.height() {
            return;
        }
        self.position += Vec2::new(0., 1.) * FIGHTER_SPEED_FACTOR;
    }
    pub async fn shift_left_up(&mut self) {
        if self.position.x <= 0. || self.position.y < 0. {
            return;
        }
        self.position -= Vec2::new(1., 1.) * FIGHTER_SPEED_FACTOR;
    }
    pub async fn shift_left_down(&mut self) {
        if self.position.x <= 0. || self.position.y > screen_height() - self.texture.height() {
            return;
        }
        self.position += Vec2::new(-1., 1.) * FIGHTER_SPEED_FACTOR;
    }
    pub async fn shift_right_up(&mut self) {
        if self.position.x > screen_width() - self.texture.width() || self.position.y < 0. {
            return;
        }
        self.position += Vec2::new(1., -1.) * FIGHTER_SPEED_FACTOR;
    }
    pub async fn shift_right_down(&mut self) {
        if self.position.x > screen_width() - self.texture.width()
            || self.position.y > screen_height() - self.texture.height()
        {
            return;
        }
        self.position += Vec2::new(1., 1.) * FIGHTER_SPEED_FACTOR;
    }

    pub async fn out_of_ammo(&self) -> bool {
        self.ammo_count == 0
    }

    pub async fn draw(&self) {
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(self.texture.width(), self.texture.height())),
            ..Default::default()
        };
        draw_texture_ex(
            self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            params,
        );
    }
}
