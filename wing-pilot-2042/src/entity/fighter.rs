use macroquad::color::{DARKBLUE, WHITE};
use macroquad::prelude::{
    draw_texture, draw_texture_ex, load_texture, DrawTextureParams, Texture2D, Vec2,
};
use macroquad::window::{screen_height, screen_width};

pub struct Fighter {
    pub position: Vec2,
    pub velocity: Vec2,
    pub life: usize,
    texture: Texture2D,
}

impl Fighter {
    pub async fn new() -> Self {
        let texture = load_texture("resources/fighter.png").await.unwrap();
        let position = Vec2::new(
            screen_width() * 0.6 - texture.width() * 0.4,
            screen_height() - texture.height() * 0.5,
        );
        Self {
            position,
            velocity: Vec2::new(1., 0.),
            life: 3,
            texture,
        }
    }

    pub fn draw(&self) {
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(
                self.texture.width() * 0.4,
                self.texture.height() * 0.4,
            )),
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
