use std::time::Instant;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Monochrome Bird", 800, 640)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let _canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let _event_pump = sdl_context.event_pump()?;
    let mut game = Game::new();
    let _last_update = Instant::now();

    loop {
        match game.state {
            GameState::Crashed => {}
            GameState::ExitGame => break,
            GameState::MainMenu => {}
            GameState::NewGame => {
                game = Game::new();
                game.state = GameState::Playing;
                continue;
            }
            GameState::Playing => {}
        }
    }

    Ok(())
}

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
}

impl Game {
    pub fn new() -> Self {
        Self {
            point: 0,
            state: GameState::MainMenu,
        }
    }
}
