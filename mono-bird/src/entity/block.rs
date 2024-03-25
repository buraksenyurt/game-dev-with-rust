use crate::entity::{Drawable, Entity};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Block {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    x_velocity: i32,
}

impl Entity for Block {
    fn update(&mut self, delta_time: f32) {
        self.x += (self.x_velocity as f32 * delta_time) as i32;
    }

    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

impl Drawable for Block {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::WHITE);
        let rect = Rect::new(self.x, self.y, self.width, self.height);
        canvas.fill_rect(rect).unwrap()
    }
}
