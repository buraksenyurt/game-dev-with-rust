use crate::constants::*;
use crate::utility::*;
use rand::Rng;
use sdl2::rect::Point;
use std::cmp;

pub struct Game {
    pub mountain_points: Vec<Point>,
    pub landing_areas: Vec<(Point, Point, Point, Point)>,
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

        let mut landing_areas = Vec::new();
        let mut used_x_positions = Vec::new();
        while landing_areas.len() < 2 {
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

            landing_areas.push((
                Point::new(landing_start, platform_height),
                Point::new(landing_end, platform_height),
                Point::new(landing_start, l_leg),
                Point::new(landing_end, r_leg),
            ));
        }

        Self {
            mountain_points,
            landing_areas,
        }
    }
}
