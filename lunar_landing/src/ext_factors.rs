use crate::entity::Shuttle;
use crate::math::Vector;
use rand::{thread_rng, Rng};

pub struct ExternalFactors {}

impl ExternalFactors {
    pub fn toss_randomly(&self, shuttle: &mut Shuttle, x_limits: Vector, delta_time: f32) {
        let mut rng = thread_rng();
        if rng.gen_range(0..100) < 2 {
            if rng.gen_bool(0.5) {
                shuttle.velocity.x += rng.gen_range(x_limits.x..x_limits.y) * delta_time;
            } else {
                shuttle.velocity.x -= rng.gen_range(x_limits.x..x_limits.y) * delta_time;
            }
        }
    }
}
