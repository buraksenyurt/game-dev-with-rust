use rusty_engine::prelude::*;

struct GameState {}

fn main() {
    let mut game = Game::new();


    game.add_logic(game_logic);
    game.run(GameState {});
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {

}