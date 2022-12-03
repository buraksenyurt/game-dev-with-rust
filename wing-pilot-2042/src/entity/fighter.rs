use crate::entity::bullet::Bullet;
use crate::entity::owner::Owner;
use macroquad::color::WHITE;
use macroquad::prelude::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D, Vec2};
use macroquad::window::{screen_height, screen_width};

pub struct Fighter {
    pub position: Vec2,
    pub life: usize,
    texture: Texture2D,
    pub ammo_count: usize,
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
            texture,
            ammo_count: 100,
        }
    }

    fn get_left_muzzle(&self) -> Vec2 {
        Vec2::new(
            self.position.x + self.texture.width() * 0.2,
            self.position.y,
        )
    }

    fn get_right_muzzle(&self) -> Vec2 {
        Vec2::new(
            self.position.x + (self.texture.width() - self.texture.width() * 0.2),
            self.position.y,
        )
    }

    pub fn spawn_bullets(&self) -> Vec<Bullet> {
        let lm = Self::get_left_muzzle(self);
        let rm = Self::get_right_muzzle(self);
        let bullet_1 = Bullet::spawn(Owner::Fighter, lm);
        let bullet_2 = Bullet::spawn(Owner::Fighter, rm);
        vec![bullet_1, bullet_2]
    }

    pub fn shift_left(&mut self) {
        if self.position.x <= 0. {
            return;
        }
        self.position -= Vec2::new(1., 0.) * 2.;
    }
    pub fn shift_right(&mut self) {
        if self.position.x >= screen_width() - self.texture.width() {
            return;
        }
        self.position += Vec2::new(1., 0.) * 2.;
    }
    pub fn shift_up(&mut self) {
        if self.position.y < 0. {
            return;
        }
        self.position -= Vec2::new(0., 1.) * 2.;
    }
    pub fn shift_down(&mut self) {
        if self.position.y > screen_height() - self.texture.height() {
            return;
        }
        self.position += Vec2::new(0., 1.) * 2.;
    }
    pub fn shift_left_up(&mut self) {
        if self.position.x <= 0. || self.position.y < 0. {
            return;
        }
        self.position -= Vec2::new(1., 1.) * 2.;
    }
    pub fn shift_left_down(&mut self) {
        if self.position.x <= 0. || self.position.y > screen_height() - self.texture.height() {
            return;
        }
        self.position += Vec2::new(-1., 1.) * 2.;
    }
    pub fn shift_right_up(&mut self) {
        if self.position.x > screen_width() - self.texture.width() || self.position.y < 0. {
            return;
        }
        self.position += Vec2::new(1., -1.) * 2.;
    }
    pub fn shift_right_down(&mut self) {
        if self.position.x > screen_width() - self.texture.width()
            || self.position.y > screen_height() - self.texture.height()
        {
            return;
        }
        self.position += Vec2::new(1., 1.) * 2.;
    }

    pub fn draw(&self) {
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
