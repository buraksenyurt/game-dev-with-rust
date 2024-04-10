use crate::entity::{Drawable, Updatable};
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Ufo {
    pub x: i32,
    pub y: i32,
    pub velocity: f32,
    pub width: u32,
    pub height: u32,
}

impl Ufo {
    pub fn new(x: i32, y: i32, velocity: f32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            velocity,
            width,
            height,
        }
    }
}

impl Updatable for Ufo {
    fn update(&mut self, delta_time: f32) {
        self.x += (self.velocity * delta_time) as i32;
        self.y += (self.velocity * delta_time) as i32;
    }
}

impl Drawable for Ufo {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::BLACK);
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture("assets/owl.png").unwrap();
        let rect = Rect::new(self.x, self.y, self.width, self.height);

        canvas.copy(&texture, None, Some(rect)).unwrap();
    }
}
