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
        let base_x = point.x + velocity.x;
        let base_y = point.y + velocity.y;
        let leg_y = point.y + SHUTTLE_HEAD_WIDTH * 2 + velocity.y;

        canvas.set_draw_color(color);
        canvas.draw_rect(Rect::new(
            base_x,
            base_y,
            SHUTTLE_HEAD_WIDTH as u32,
            SHUTTLE_HEAD_WIDTH as u32,
        ))?;

        let half_width = SHUTTLE_HEAD_WIDTH / 2;
        let quarter_width = SHUTTLE_HEAD_WIDTH / 4;

        // Sol bacak
        let left_leg_start = Point::new(base_x, base_y + SHUTTLE_HEAD_WIDTH);
        let left_leg_end = Point::new(base_x - half_width, leg_y);
        canvas.draw_line(left_leg_start, left_leg_end)?;

        // Sol ayak
        canvas.draw_line(
            left_leg_end,
            Point::new(left_leg_end.x - quarter_width, leg_y),
        )?;

        // Sağ bacak
        let right_leg_start = Point::new(base_x + SHUTTLE_HEAD_WIDTH, base_y + SHUTTLE_HEAD_WIDTH);
        let right_leg_end = Point::new(base_x + SHUTTLE_HEAD_WIDTH + half_width, leg_y);
        canvas.draw_line(right_leg_start, right_leg_end)?;

        // Sağ ayak
        canvas.draw_line(
            right_leg_end,
            Point::new(right_leg_end.x + quarter_width, leg_y),
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
