use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Radar Jam", "BSŞ");
    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

struct GameState {
    current_r: f32
}

impl GameState {
    fn new(_ctx: &mut Context) -> GameResult<GameState> {
        Ok(GameState { current_r: 0.0 })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.current_r = self.current_r % 500.0 + 0.5;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(2.),
            vec2(100., 100.),
            self.current_r,
            0.5,
            Color::from_rgb(0, 143, 17),
        )?;
        canvas.draw(&circle, Vec2::new(0., 0.));
        canvas.finish(ctx)?;

        Ok(())
    }
}
