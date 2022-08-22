use ggez::event::EventHandler;
use ggez::graphics::{draw, DrawParam};
use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};

pub struct Game {}

impl Game {
    pub fn new(_context: &mut Context) -> Self {
        Game {}
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let screen = graphics::drawable_size(&ctx);

        graphics::clear(ctx, graphics::Color::from_rgb(17, 65, 143));

        // draw vertical rectangle
        let column = graphics::Rect::new(0., 0., 25., 200.);
        let column_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            column,
            graphics::Color::WHITE,
        )?;

        draw(
            ctx,
            &column_mesh,
            DrawParam::new().dest(Point2 {
                x: screen.0 / 2. - 12.5,
                y: 5.,
            }),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}
