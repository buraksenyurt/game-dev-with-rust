use ggez::conf::WindowMode;
use ggez::graphics::Text;
use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};

const SCREEN_WIDTH: f32 = 400.;
const SCREEN_HEIGHT: f32 = 400.;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Radar Jam", "BSŞ").window_mode(
        WindowMode::default()
            .dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
            .resizable(false),
    );
    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

struct GameState {
    circle_r: f32,
    score: f32,
}

impl GameState {
    fn new(_ctx: &mut Context) -> GameResult<GameState> {
        Ok(GameState {
            circle_r: 0.,
            score: 0.,
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.circle_r >= SCREEN_WIDTH / 2. {
            self.circle_r = 0.;
        }
        self.circle_r = self.circle_r + 0.8;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(2.),
            vec2(SCREEN_WIDTH / 2., SCREEN_HEIGHT / 2.),
            self.circle_r,
            0.5,
            Color::from_rgb(0, 143, 17),
        )?;
        canvas.draw(&circle, Vec2::new(0., 0.));

        for i in 1..4 {
            let line = graphics::Mesh::new_line(
                ctx,
                &[
                    Vec2::new(0., i as f32 * 100.),
                    Vec2::new(SCREEN_WIDTH, i as f32 * 100.),
                ],
                2.,
                Color::from_rgb(123, 122, 121),
            )?;
            canvas.draw(&line, graphics::DrawParam::from([0., 0.]));

            let line = graphics::Mesh::new_line(
                ctx,
                &[
                    Vec2::new(i as f32 * 100.,0. ),
                    Vec2::new(i as f32 * 100.,SCREEN_HEIGHT),
                ],
                2.,
                Color::from_rgb(123, 122, 121),
            )?;
            canvas.draw(&line, graphics::DrawParam::from([0., 0.]));
        }

        let score_text = Text::new(format!("score:{}  r:{}", self.score, self.circle_r));
        canvas.draw(&score_text, graphics::DrawParam::from([0., 0.]));

        canvas.finish(ctx)?;

        Ok(())
    }
}
