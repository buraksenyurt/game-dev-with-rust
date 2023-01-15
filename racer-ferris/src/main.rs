use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    game.run(GameState::default());
}

struct Score {
    high: u32,
    current: u32,
}

struct GameState {
    score: Score,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState{
    fn default() -> Self {
        Self{
            score:Score{high:0,current:0},
            enemy_labels:Vec::new(),
            spawn_timer:Timer::from_seconds(1.,false)
        }
    }
}
