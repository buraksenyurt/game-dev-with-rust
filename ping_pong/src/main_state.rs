use crate::constants::{RACKET_H, RACKET_H_HALF, RACKET_W, RACKET_W_HALF};
use ggez::event::EventHandler;
use ggez::{graphics, Context, GameResult};

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
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from_rgb(55, 109, 93));

        let racket = graphics::Rect::new(-RACKET_W_HALF, -RACKET_H_HALF, RACKET_W, RACKET_H);
        let racket_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            racket,
            graphics::Color::WHITE,
        )?;
        graphics::draw(ctx, &racket_mesh, graphics::DrawParam::default())?;

        graphics::present(ctx)?;
        Ok(())
    }
}
