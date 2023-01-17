use rusty_engine::prelude::bevy::log::info;
use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    let veyron = game.add_sprite("Player", SpritePreset::RacingCarYellow);
    veyron.translation = Vec2::new(-200., 0.);
    veyron.rotation = EAST;
    veyron.scale = 0.5;
    //veyron.layer=1.;
    veyron.collision = true;

    let esprit = game.add_sprite("Keira", SpritePreset::RacingCarBlue);
    esprit.translation = Vec2::new(200., 0.);
    esprit.rotation = WEST;
    esprit.scale = 0.5;
    //esprit.layer=2.;
    esprit.collision = true;

    game.add_logic(game_logic);
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

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: Score {
                high: 0,
                current: 0,
            },
            enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(1., false),
        }
    }
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game_state.score.current += 1;
    // info!("Güncel skor {}", game_state.score.current);

    for event in engine.collision_events.drain(..) {
        info!("{:#?}", event);
        if event.state == CollisionState::Begin && event.pair.one_starts_with("Player") {
            for lbl in [event.pair.0, event.pair.1] {
                if lbl != "Player" {
                    engine.sprites.remove(&lbl);
                }
            }

            game_state.score.current += 1;
            info!("Oyuncu bir puan kazandı. Güncel skor: {}", game_state.score.current);
        }
    }

    let players_car = engine.sprites.get_mut("Player").unwrap();
    players_car.translation.x += 75. * engine.delta_f32;

    // let keiras_car = engine.sprites.get_mut("Keira").unwrap();
    // keiras_car.translation.x -= 50. * engine.delta_f32;
}
