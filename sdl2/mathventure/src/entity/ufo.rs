use crate::entity::{Drawable, Updatable};
use crate::factory::{AssetManager, Dimension, Location, Rectangle, Vector};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Ufo {
    pub location: Location,
    pub velocity: Vector,
    pub dimension: Dimension,
    pub name: String,
    pub hit: bool,
}

impl Ufo {
    pub fn new(location: Location, velocity: Vector, dimension: Dimension, name: String) -> Self {
        Self {
            location,
            velocity,
            dimension,
            name,
            hit: false,
        }
    }
    pub fn get_rect(&self) -> Rectangle {
        Rectangle::new(
            Location::new(self.location.x, self.location.y),
            Dimension::new(self.dimension.width, self.dimension.height),
        )
    }
}

impl Updatable for Ufo {
    fn update(&mut self, delta_time: f32) {
        self.location.x += (self.velocity.x * delta_time) as i32;
        self.location.y += (self.velocity.y * delta_time) as i32;
    }
}

impl Drawable for Ufo {
    fn draw(&self, canvas: &mut Canvas<Window>, asset_manager: &AssetManager) {
        canvas.set_draw_color(Color::BLACK);
        let texture = asset_manager.get(&self.name);
        let rect = Rect::new(
            self.location.x,
            self.location.y,
            self.dimension.width,
            self.dimension.height,
        );

        canvas.copy(texture, None, Some(rect)).unwrap();
    }
}
