use crate::factory::{Engine, GameObject, Screen};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub struct EngineBuilder {
    sdl_context: sdl2::Sdl,
    canvas: Option<Canvas<Window>>,
    event_pump: Option<EventPump>,
    game: Option<Box<dyn GameObject>>,
    fps: u32,
}

impl EngineBuilder {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        Ok(Self {
            sdl_context,
            canvas: None,
            event_pump: None,
            game: None,
            fps: 30,
        })
    }
    pub fn setup_screen(mut self, screen: Screen) -> Result<Self, String> {
        let video_subsystem = self.sdl_context.video().map_err(|e| e.to_string())?;
        let window = video_subsystem
            .window(&screen.title, screen.width, screen.height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let event_pump = self.sdl_context.event_pump().map_err(|e| e.to_string())?;

        self.canvas = Some(canvas);
        self.event_pump = Some(event_pump);
        Ok(self)
    }

    pub fn add_game(mut self, game: Box<dyn GameObject>) -> Self {
        self.game = Some(game);
        self
    }

    pub fn change_fps(mut self, fps: u32) -> Self {
        self.fps = fps;
        self
    }

    pub fn build(self) -> Result<Engine, String> {
        if self.canvas.is_none() || self.event_pump.is_none() || self.game.is_none() {
            return Err("Missing setup".into());
        }
        Ok(Engine {
            game: self.game.unwrap(),
            fps: self.fps,
            event_pump: self.event_pump.unwrap(),
            canvas: self.canvas.unwrap(),
        })
    }
}
