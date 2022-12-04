use crate::common::constants::MAX_AMMO;
use crate::entity::bullet::Bullet;
use crate::entity::fleet::Fleet;
use crate::game::state::State;

pub struct Game {
    pub state: State,
    pub bullets: Vec<Bullet>,
    pub enemy_fleet: Fleet,
    pub fighter_amount_count: usize,
}

impl Game {
    pub fn new(state: State) -> Self {
        Self {
            state,
            bullets: Vec::default(),
            enemy_fleet: Fleet::default(),
            fighter_amount_count: MAX_AMMO,
        }
    }
}
