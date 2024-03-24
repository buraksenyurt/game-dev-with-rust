use std::time::Duration;

#[derive(PartialEq)]
pub enum GameState {
    Crashed,
    ExitGame,
    MainMenu,
    NewGame,
    Playing,
}

pub struct Game {
    pub point: u64,
    pub state: GameState,
    pub delta_second: Duration,
}

impl Game {
    pub fn new() -> Self {
        Self {
            point: 0,
            state: GameState::MainMenu,
            delta_second: Duration::default(),
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
