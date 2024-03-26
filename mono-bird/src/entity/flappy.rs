use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::entity::{Drawable, Entity};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Flappy {
    pub x: i32,
    pub y: i32,
    velocity: f32,
}

impl Default for Flappy {
    fn default() -> Self {
        Self {
            x: 25,
            y: SCREEN_WIDTH as i32 / 2,
            velocity: 0.5,
        }
    }
}
impl Entity for Flappy {
    fn update(&mut self, delta_time: f32) {
        self.y += (self.velocity * delta_time) as i32;
    }

    fn position(&self) -> (i32, i32) {
        (10, self.y)
    }

    fn set_position(&mut self, _x: i32, y: i32) {
        self.x = 10;
        self.y = y;
    }
}

impl Drawable for Flappy {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::WHITE);
        let rect = Rect::new(self.x, self.y, 25, 25);
        canvas.fill_rect(rect).unwrap()
    }
}
