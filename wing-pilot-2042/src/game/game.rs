use crate::game::state::State;

pub struct Game {
    pub state: State,
}

impl Game {
    pub fn new(state: State) -> Self {
        Self { state }
    }
}
