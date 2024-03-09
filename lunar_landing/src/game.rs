use crate::constants::*;
use crate::entity::*;
use crate::utility::*;
use rand::{thread_rng, Rng};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use std::cmp;

#[derive(PartialEq)]
pub struct Game {
    pub mountain_points: Vec<Point>,
    pub landing_platforms: Vec<LandingPlatform>,
    pub meteors: Vec<Meteor>,
    pub state: GameState,
}

impl Game {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let ground_level = HEIGHT - 100;

        let mut mountain_points = Vec::new();
        let mut x = 0;
        while x < WIDTH {
            let peak_height = rng.gen_range(ground_level - 150..ground_level);
            mountain_points.push(Point::new(x, peak_height));
            x += rng.gen_range(80..120);
        }
        mountain_points.push(Point::new(
            WIDTH,
            rng.gen_range(ground_level - 150..ground_level),
        ));

        let mut platforms = Vec::new();
        let mut used_x_positions = Vec::new();
        while platforms.len() < 2 {
            let mut landing_start = rng.gen_range(0..WIDTH - 50);
            while used_x_positions
                .iter()
                .any(|&pos| (landing_start..landing_start + 50).contains(&pos))
            {
                landing_start = rng.gen_range(0..WIDTH - 50);
            }
            let landing_end = cmp::min(landing_start + 65, WIDTH);
            used_x_positions.extend(landing_start..landing_end);

            let platform_height = rng.gen_range(ground_level - 150..ground_level - 100);
            let l_leg = find_ground_height(&mountain_points, landing_start);
            let r_leg = find_ground_height(&mountain_points, landing_end);

            platforms.push(LandingPlatform::new(
                Point::new(landing_start, platform_height),
                Point::new(landing_end, platform_height),
                Point::new(landing_start, l_leg),
                Point::new(landing_end, r_leg),
            ));
        }
        let mut meteors = Vec::new();
        for _ in 1..=MAX_METEOR_COUNT {
            let m = Self::spawn_random_meteor();
            meteors.push(m);
        }

        Self {
            mountain_points,
            landing_platforms: platforms,
            meteors,
            state: GameState::Menu,
        }
    }
    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        for i in 0..self.mountain_points.len() - 1 {
            let start = self.mountain_points[i];
            let end = self.mountain_points[i + 1];
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.draw_line(start, end)?;
        }

        for p in &self.landing_platforms {
            p.draw(canvas)?;
        }

        for m in &self.meteors {
            m.draw(canvas)?;
        }

        Ok(())
    }

    pub fn move_meteors(&mut self, delta_time: f32) {
        let mut rng = thread_rng();
        for m in self.meteors.iter_mut() {
            m.rot_angle += 25. * delta_time as f64;
            m.velocity.y += rng.gen_range(10.0..15.) * delta_time;
            match m.kind {
                MeteorType::LeftBottomCorner => {
                    m.velocity.x -= rng.gen_range(10.0..15.) * delta_time;
                }
                MeteorType::RightBottomCorner => {
                    m.velocity.x += rng.gen_range(10.0..15.) * delta_time;
                }
                MeteorType::Vertical => {
                    m.velocity.y += 10. * delta_time;
                }
            }
            m.mark_range();
        }
    }

    pub fn check_out_of_ranges(&mut self) {
        self.meteors.retain(|m| m.in_range);
    }

    fn spawn_random_meteor() -> Meteor {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..WIDTH - WIDTH / 4);
        let y = rng.gen_range(-100..-10);
        let side_count = rng.gen_range(6..10);
        let radius = rng.gen_range(10..20);
        let angle = rng.gen_range(10..30) as f64;
        Meteor::new(Point::new(x, y), side_count, radius, angle, true)
    }

    pub fn respawn_meteors(&mut self) {
        if self.meteors.is_empty() {
            for _ in 1..=MAX_METEOR_COUNT {
                let m = Self::spawn_random_meteor();
                self.meteors.push(m);
            }
        }
    }

    pub fn check_meteor_shuttle_collisions(&self, shuttle: &Shuttle) -> bool {
        for m in self.meteors.iter() {
            if shuttle.check_collision_with_meteor(m) {
                return true;
            }
        }
        false
    }
}
