use crate::constants::*;
use crate::entity::meteor::Meteor;
use crate::entity::vector::Vector;
use crate::game::Game;
use rand::{thread_rng, Rng};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(PartialEq)]
pub struct Shuttle {
    pub position: Point,
    pub fuel_level: i32,
    pub velocity: Vector,
}

impl Shuttle {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(50..WIDTH - 50);
        let y = rng.gen_range(10..WIDTH / 8);
        let position = Point::new(x, y);
        Self {
            position,
            fuel_level: DEFAULT_FUEL_LEVEL,
            velocity: Vector::default(),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, color: Color) -> Result<(), String> {
        let point = self.position;
        let velocity = self.velocity.to_point();
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

    pub fn calculate_foot_points(&self) -> ((Point, Point), (Point, Point)) {
        let foot_width = SHUTTLE_HEAD_WIDTH / 2;
        let foot_y = self.position.y + SHUTTLE_HEAD_WIDTH * 2 + self.velocity.y as i32;
        let left_foot_start = Point::new(
            self.position.x - foot_width + self.velocity.x as i32,
            foot_y,
        );
        let left_foot_end = Point::new(self.position.x + self.velocity.x as i32, foot_y);
        let right_foot_start = Point::new(self.position.x + self.velocity.x as i32, foot_y);
        let right_foot_end = Point::new(
            self.position.x + foot_width + self.velocity.x as i32,
            foot_y,
        );
        (
            (left_foot_start, left_foot_end),
            (right_foot_start, right_foot_end),
        )
    }

    pub fn is_landed(&self, game: &Game) -> bool {
        let mut is_landed = false;
        for lp in &game.landing_platforms {
            if lp.check_collision(self) {
                is_landed = true;
                break;
            }
        }
        is_landed
    }

    pub fn is_fuel_critical(&self) -> bool {
        self.fuel_level <= DEFAULT_FUEL_LEVEL / 4
    }

    pub fn is_low_altitude(&self) -> bool {
        self.position.y + self.velocity.y as i32 >= HEIGHT - HEIGHT / 4
    }

    pub fn check_collision_with_meteor(&self, meteor: &Meteor) -> bool {
        let shuttle_x = self.position.x as f32 + self.velocity.x + (SHUTTLE_HEAD_WIDTH / 2) as f32;
        let shuttle_y = self.position.y as f32 + self.velocity.y + (SHUTTLE_HEAD_WIDTH / 2) as f32;

        let meteor_x = meteor.center.x as f32 + meteor.velocity.x;
        let meteor_y = meteor.center.y as f32 + meteor.velocity.y;

        let dist_x = shuttle_x - meteor_x;
        let dist_y = shuttle_y - meteor_y;
        let euclidean_distance = (dist_x.powi(2) + dist_y.powi(2)).sqrt();

        let shuttle_radius =
            ((SHUTTLE_HEAD_WIDTH.pow(2) + SHUTTLE_HEAD_WIDTH.pow(2)) as f32).sqrt() / 2.0;
        let meteor_radius = meteor.radius as f32;

        euclidean_distance <= (shuttle_radius + meteor_radius)
    }

    pub fn toss_randomly(&mut self, x_limits: Vector, delta_time: f32) {
        let mut rng = thread_rng();
        if rng.gen_range(0..100) < 2 {
            if rng.gen_bool(0.5) {
                self.velocity.x += rng.gen_range(x_limits.x..x_limits.y) * delta_time;
            } else {
                self.velocity.x -= rng.gen_range(x_limits.x..x_limits.y) * delta_time;
            }
        }
    }
}
