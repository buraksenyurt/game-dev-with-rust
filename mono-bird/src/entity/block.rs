use crate::entity::flappy::Flappy;
use crate::entity::{Drawable, Entity};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Block {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub x_velocity: f32,
    pub direction: BlockDirection,
}

impl Block {
    pub fn intersects(&self, other: &Flappy) -> bool {
        self.x < other.x + 25
            && self.x + self.width as i32 > other.x
            && self.y < other.y + 25
            && self.y + self.height as i32 > other.y
    }
}

impl Entity for Block {
    fn update(&mut self, delta_time: f32) {
        self.x += (self.x_velocity * delta_time) as i32;
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

pub enum BlockDirection {
    BottomToUp,
    TopToBottom,
}
