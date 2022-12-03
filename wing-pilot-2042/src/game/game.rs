use crate::entity::bullet::Bullet;
use crate::game::state::State;

pub struct Game {
    pub state: State,
    pub bullets: Vec<Bullet>,
}

impl Game {
    pub fn new(state: State) -> Self {
        Self {
            state,
            bullets: Vec::new(),
        }
    }
}
