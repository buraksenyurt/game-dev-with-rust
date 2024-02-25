use crate::constants::WIDTH;
use crate::entity::{Meteor, Shuttle};
use rand::{thread_rng, Rng};
use sdl2::rect::Point;

pub struct ExternalFactors {
    pub min_x: f32,
    pub max_x: f32,
}

impl ExternalFactors {
    pub fn new(min_x: f32, max_x: f32) -> Self {
        Self { min_x, max_x }
    }
    pub fn toss_randomly(&self, shuttle: &mut Shuttle) {
        let mut rng = thread_rng();
        if rng.gen_range(0..100) < 2 {
            if rng.gen_bool(0.5) {
                shuttle.velocity.x += rng.gen_range(self.min_x..self.max_x);
            } else {
                shuttle.velocity.x -= rng.gen_range(self.min_x..self.max_x);
            }
        }
    }

    pub fn spawn_random_meteor(&self) -> Meteor {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..WIDTH - WIDTH / 4);
        let y = rng.gen_range(-100..-10);
        let side_count = rng.gen_range(6..10);
        let radius = rng.gen_range(10..20);
        let angle = rng.gen_range(10..30) as f64;
        Meteor::new(Point::new(x, y), side_count, radius, angle)
    }
}
