use crate::entity::Shuttle;
use rand::{thread_rng, Rng};

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
}
