use crate::constants::{
    BALL_SIZE, BALL_SIZE_HALF, RACKET_H, RACKET_H_HALF, RACKET_W, RACKET_W_HALF,
};
use ggez::event::EventHandler;
use ggez::graphics::DrawParam;
use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};

pub struct MainState {
    p1_position: Point2<f32>,
    p2_position: Point2<f32>,
    ball_position: Point2<f32>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (scr_width, scr_height) = graphics::drawable_size(ctx);
        //println!("{}X{}", scr_width, scr_height);
        let (scr_width_half, scr_height_half) = (scr_width * 0.5, scr_height * 0.5);
        MainState {
            p1_position: Point2 {
                x: RACKET_W_HALF,
                y: scr_height_half,
            },
            p2_position: Point2 {
                x: scr_width - RACKET_W_HALF,
                y: scr_height_half,
            },
            ball_position: Point2 {
                x: scr_width_half,
                y: scr_height_half,
            },
        }
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

        graphics::draw(ctx, &racket_mesh, DrawParam::new().dest(self.p1_position))?;
        graphics::draw(ctx, &racket_mesh, DrawParam::new().dest(self.p2_position))?;

        let ball = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball,
            graphics::Color::WHITE,
        )?;

        graphics::draw(ctx, &ball_mesh, DrawParam::new().dest(self.ball_position))?;

        graphics::present(ctx)?;
        Ok(())
    }
}
