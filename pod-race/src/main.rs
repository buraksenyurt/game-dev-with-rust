use rusty_engine::prelude::*;

const PLAYER_SPEED: f32 = 500.;
const ROAD_SPEED: f32 = 400.;

struct GameState {
    health: u8,
    lost: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            health: 5,
            lost: false,
        }
    }
}

fn main() {
    let mut game = Game::new();

    let player1 = game.add_sprite("player1", "sprite/player.png");
    player1.translation.x = -500.;
    player1.layer = 10.;
    player1.collision = true;
    game.audio_manager.play_music("jamsi4.ogg", 0.2);
    for i in 0..10 {
        let line = game.add_sprite(format!("line_{}", i), "sprite/barrier_white.png");
        line.scale = 0.1;
        line.translation.x = -600. + 150. * i as f32;
    }

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let mut direction = 0.;
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        direction += 1.;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        direction -= 1.;
    }
    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15;
    if player1.translation.y < -360. || player1.translation.y > 360. {
        game_state.health = 0;
    }

    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("line") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -675. {
                sprite.translation.x += 1500.
            }
        }
    }
}
