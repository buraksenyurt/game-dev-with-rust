use rusty_engine::prelude::bevy::log::info;
use rusty_engine::prelude::*;

const PLAYER_MOVEMENT_SPEED: f32 = 100.;

fn main() {
    let mut game = Game::new();

    let walle = game.add_sprite("Player", "robot.png");
    walle.translation = Vec2::new(-200., 0.);
    walle.rotation = EAST;
    walle.scale = 1.;
    //veyron.layer=1.;
    walle.collision = true;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

struct Score {
    //high: u32,
    current: u32,
}

struct GameState {
    score: Score,
    battery_index: i32,
    //enemy_labels: Vec<String>,
    //spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: Score {
                //high: 0,
                current: 0,
            },
            //enemy_labels: Vec::new(),
            //spawn_timer: Timer::from_seconds(1., false),
            battery_index: 0,
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
            info!(
                "Oyuncu bir puan kazandı. Güncel skor: {}",
                game_state.score.current
            );
        }
    }

    let player = engine.sprites.get_mut("Player").unwrap();

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        player.translation.y += PLAYER_MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        player.translation.y -= PLAYER_MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        player.translation.x -= PLAYER_MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        player.translation.x += PLAYER_MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(mouse_location) = engine.mouse_state.location() {
            let battery_label = format!("BTRY_{}", game_state.battery_index);
            game_state.battery_index += 1;

            let battery = engine.add_sprite(battery_label.clone(), "battery.png");
            battery.translation = mouse_location;
            //battery.rotation = WEST;
            battery.scale = 0.8;
            //esprit.layer=2.;
            battery.collision = true;
        }
    }
}
