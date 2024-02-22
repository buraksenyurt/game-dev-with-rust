use crate::constants::*;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::video::Window;

pub struct Shuttle {
    pub position: Point,
    pub fuel_level: i32,
}

impl Shuttle {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(50..WIDTH - 50);
        let y = rng.gen_range(10..100);
        let position = Point::new(x, y);
        Self {
            position,
            fuel_level: DEFAULT_FUEL_LEVEL,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        color: Color,
        velocity: Point,
    ) -> Result<(), String> {
        let point = self.position;

        canvas.set_draw_color(color);
        canvas.draw_rect(Rect::new(
            point.x + velocity.x,
            point.y + velocity.y,
            SHUTTLE_HEAD_WIDTH as u32,
            SHUTTLE_HEAD_WIDTH as u32,
        ))?;
        canvas.draw_line(
            Point::new(
                point.x + velocity.x,
                point.y + SHUTTLE_HEAD_WIDTH + velocity.y,
            ),
            Point::new(
                point.x - SHUTTLE_HEAD_WIDTH / 2 + velocity.x,
                point.y + SHUTTLE_HEAD_WIDTH * 2 + velocity.y,
            ),
        )?;
        canvas.draw_line(
            Point::new(
                point.x + SHUTTLE_HEAD_WIDTH + velocity.x,
                point.y + SHUTTLE_HEAD_WIDTH + velocity.y,
            ),
            Point::new(
                point.x + SHUTTLE_HEAD_WIDTH + SHUTTLE_HEAD_WIDTH / 2 + velocity.x,
                point.y + SHUTTLE_HEAD_WIDTH * 2 + velocity.y,
            ),
        )?;

        Ok(())
    }
}

pub struct LandingPlatform {
    pub p1: Point,
    pub p2: Point,
    pub left_leg: Point,
    pub right_leg: Point,
}

impl LandingPlatform {
    pub fn new(p1: Point, p2: Point, left_leg: Point, right_leg: Point) -> Self {
        Self {
            p1,
            p2,
            left_leg,
            right_leg,
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_line(self.p1, self.p2)?;
        canvas.draw_line(self.p1, self.left_leg)?;
        canvas.draw_line(self.p2, self.right_leg)?;

        Ok(())
    }
}
