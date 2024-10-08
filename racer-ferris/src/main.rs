use rand::{thread_rng, Rng};
use rusty_engine::prelude::bevy::log::info;
use rusty_engine::prelude::*;

const PLAYER_MOVEMENT_SPEED: f32 = 100.;
const MAX_WARP_COUNT: u8 = 3;

fn main() {
    let mut game = Game::new();

    game.window_settings(WindowDescriptor {
        title: "Eat the Garbage!".to_string(),
        width: 1366.,
        height: 768.,
        ..Default::default()
    });

    game.audio_manager
        .play_music("Wagner_The_Valkyrie.ogg", 0.1);

    let walle = game.add_sprite("Player", "robot.png");
    walle.translation = Vec2::new(-200., 0.);
    walle.rotation = EAST;
    walle.scale = 1.;
    //veyron.layer=1.;
    walle.collision = true;

    let text_high_score = game.add_text("lblHighScore", "High Score : 0");
    text_high_score.translation = Vec2::new(420., 320.);

    let text_current_score = game.add_text("lblCurrentScore", "Score : 0");
    text_current_score.translation = Vec2::new(-420., 320.);

    game.add_logic(game_logic);
    game.run(GameState::default());
}

struct Score {
    high: u32,
    current: u32,
}

struct GameState {
    score: Score,
    battery_index: i32,
    //enemy_labels: Vec<String>,
    spawn_timer: Timer,
    booster: SpeedPowerup,
}

struct SpeedPowerup {
    timer: Timer,
    max: u8,
    value: f32,
    is_active: bool,
}

impl SpeedPowerup {
    pub fn new() -> Self {
        Self {
            max: MAX_WARP_COUNT,
            value: 1.,
            is_active: false,
            timer: Timer::from_seconds(3., true),
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: Score {
                high: 0,
                current: 0,
            },
            //enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(2., true),
            battery_index: 0,
            booster: SpeedPowerup::new(),
        }
    }
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game_state.score.current += 1;
    // info!("Güncel skor {}", game_state.score.current);

    if engine.keyboard_state.just_pressed(KeyCode::Q) {
        engine.should_exit = true;
    }

    set_texts_position(engine);

    for event in engine.collision_events.drain(..) {
        info!("{:#?}", event);
        if event.state == CollisionState::Begin && event.pair.one_starts_with("Player") {
            for lbl in [event.pair.0, event.pair.1] {
                if lbl != "Player" {
                    engine.sprites.remove(&lbl);
                }
            }

            game_state.score.current += 1;
            let text_current_score = engine.texts.get_mut("lblCurrentScore").unwrap();
            text_current_score.value = format!("Score: {}", game_state.score.current);
            // info!(
            //     "Oyuncu bir puan kazandı. Güncel skor: {}",
            //     game_state.score.current
            // );

            if game_state.score.current > game_state.score.high {
                game_state.score.high = game_state.score.current;
                let text_high_score = engine.texts.get_mut("lblHighScore").unwrap();
                text_high_score.value = format!("High Score: {}", game_state.score.high);
            }
            engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.2);
        }
    }

    let player = engine.sprites.get_mut("Player").unwrap();

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        player.translation.y += PLAYER_MOVEMENT_SPEED * game_state.booster.value * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        player.translation.y -= PLAYER_MOVEMENT_SPEED * game_state.booster.value * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        player.translation.x -= PLAYER_MOVEMENT_SPEED * game_state.booster.value * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        player.translation.x += PLAYER_MOVEMENT_SPEED * game_state.booster.value * engine.delta_f32;
    }

    if game_state.booster.max > 0 && engine.keyboard_state.just_pressed(KeyCode::W) {
        game_state.booster.is_active = true;
        game_state.booster.value = 3.;
        game_state.booster.timer.reset();
        game_state.booster.max -= 1;
    }

    if game_state.booster.timer.tick(engine.delta).just_finished() && game_state.booster.is_active {
        game_state.booster.is_active = false;
        game_state.booster.value = 1.;
    }

    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let battery_label = format!("BTRY_{}", game_state.battery_index);
        game_state.battery_index += 1;
        let battery = engine.add_sprite(battery_label.clone(), "battery.png");
        battery.translation.x = thread_rng().gen_range(-420. ..420.);
        battery.translation.y = thread_rng().gen_range(-320. ..320.);
        battery.scale = 0.8;
        battery.collision = true;
    }

    if engine.keyboard_state.pressed(KeyCode::R) {
        game_state.score.current = 0;
        game_state.booster = SpeedPowerup::new();
        let text_current_score = engine.texts.get_mut("lblCurrentScore").unwrap();
        text_current_score.value = "Score: 0".to_string();
    }
}

fn set_texts_position(engine: &mut Engine) {
    let score = engine.texts.get_mut("lblCurrentScore").unwrap();
    score.translation.x = engine.window_dimensions.x / 2. - 100.;
    score.translation.y = engine.window_dimensions.y / 2. - 50.;
    let high_score = engine.texts.get_mut("lblHighScore").unwrap();
    high_score.translation.x = -engine.window_dimensions.x / 2. + 100.;
    high_score.translation.y = engine.window_dimensions.y / 2. - 50.;
}
