use crate::entity::{Drawable, Updatable};
use crate::resources::{TextureManager, Velocity};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Ufo {
    pub x: i32,
    pub y: i32,
    pub velocity: Velocity,
    pub width: u32,
    pub height: u32,
    pub name: String,
}

impl Ufo {
    pub fn new(x: i32, y: i32, velocity: Velocity, width: u32, height: u32, name: String) -> Self {
        Self {
            x,
            y,
            velocity,
            width,
            height,
            name,
        }
    }
}

impl Updatable for Ufo {
    fn update(&mut self, delta_time: f32) {
        self.x += (self.velocity.x as f32 * delta_time) as i32;
        self.y += (self.velocity.y as f32 * delta_time) as i32;
    }
}

impl Drawable for Ufo {
    fn draw(&self, canvas: &mut Canvas<Window>, texture_manager: &TextureManager) {
        canvas.set_draw_color(Color::BLACK);
        let texture = texture_manager.get(&self.name);
        let rect = Rect::new(self.x, self.y, self.width, self.height);

        canvas.copy(texture, None, Some(rect)).unwrap();
    }
}
