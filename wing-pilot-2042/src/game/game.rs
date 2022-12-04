use crate::entity::bullet::Bullet;
use crate::entity::fighter::Fighter;
use crate::game::state::State;

pub struct Game {
    pub state: State,
    pub bullets: Vec<Bullet>,
    pub fighter_ammount_count: usize,
}

impl Game {
    pub fn new(state: State) -> Self {
        Self {
            state,
            bullets: Vec::new(),
            fighter_ammount_count: 100,
        }
    }
}
