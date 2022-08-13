use ggez::event::EventHandler;
use ggez::{event, Context, GameError, GameResult};

pub struct MainState {}

impl MainState {
    pub fn new(_ctx: &mut Context) -> Self {
        MainState {}
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}
