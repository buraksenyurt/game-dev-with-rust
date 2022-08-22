use ggez::event::EventHandler;
use ggez::{graphics, Context, GameError};

pub struct Game {}

impl Game {
    pub fn new(_context: &mut Context) -> Self {
        Game {}
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx, graphics::Color::from_rgb(17, 65, 143));

        graphics::present(ctx)?;
        Ok(())
    }
}
