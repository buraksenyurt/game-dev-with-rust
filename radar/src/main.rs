//! Radar Game
//! The aim of this game is to shoot vehicles that appear on the radar.
//! However, it is necessary to pay attention to the distinction between friend and foe.
//! Hitting allied forces is minus points.//
// In general, the basic functionalities of ggez are covered.

use ggez::conf::WindowMode;
use ggez::graphics::Text;
use ggez::mint::Point2;
use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};
use rand::Rng;
use std::f32::consts::PI;

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
    angle: f32,
    score: f32,
    mouse: Mouse,
    enemy_x: f32,
    enemy_y: f32,
    enemy_is_live: bool,
    enemy_spawned: bool,
}

struct Mouse(Point2<f32>);

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        Ok(GameState {
            circle_r: 0.,
            angle: 0.,
            score: 0.,
            mouse: Mouse(ctx.mouse.position()),
            enemy_x: 0.,
            enemy_y: 0.,
            enemy_is_live: false,
            enemy_spawned: false,
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Increase radius of radar circle
        if self.circle_r >= SCREEN_WIDTH / 2. {
            self.circle_r = 0.;
        }
        self.circle_r = &self.circle_r + 0.8;
        self.angle = &self.angle + PI / 90.;
        // Calculate random enemey points
        if ctx.time.ticks() % 120 == 0 {
            let mut rng = rand::thread_rng();
            let angle: f32 = rng.gen_range(-360.0..360.);
            self.enemy_x = SCREEN_WIDTH * 0.5 + &self.circle_r * angle.cos();
            self.enemy_y = SCREEN_HEIGHT * 0.5 + &self.circle_r * angle.sin();
            self.enemy_spawned = true;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Radar circle draw
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(2.),
            vec2(SCREEN_WIDTH / 2., SCREEN_HEIGHT / 2.),
            self.circle_r,
            0.5,
            Color::from_rgb(0, 143, 17),
        )?;
        canvas.draw(&circle, Vec2::new(0., 0.));

        // draw circle line
        let circle_line = graphics::Mesh::new_line(
            ctx,
            &[
                Vec2::new(SCREEN_WIDTH / 2., SCREEN_HEIGHT / 2.),
                Vec2::new(
                    SCREEN_WIDTH / 2. + &self.circle_r * &self.angle.cos(),
                    SCREEN_HEIGHT / 2. + &self.circle_r * &self.angle.sin(),
                ),
            ],
            2.,
            Color::from_rgb(0, 143, 17),
        )?;
        canvas.draw(&circle_line, Vec2::new(0., 0.));

        // Grid lines draw
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
                    Vec2::new(i as f32 * 100., 0.),
                    Vec2::new(i as f32 * 100., SCREEN_HEIGHT),
                ],
                2.,
                Color::from_rgb(123, 122, 121),
            )?;
            canvas.draw(&line, graphics::DrawParam::from([0., 0.]));
        }

        // Enemey draw
        if self.enemy_spawned {
            let enemy_circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                vec2(self.enemy_x, self.enemy_y),
                10.,
                0.1,
                Color::WHITE,
            )?;
            canvas.draw(&enemy_circle, graphics::DrawParam::from([0., 0.]));
        }

        // Informal text draw
        let score_text = Text::new(format!("score:{}  r:{}", self.score, self.circle_r));
        canvas.draw(&score_text, graphics::DrawParam::from([0., 0.]));

        // Mouse cursor draw
        let mouse_line = graphics::Mesh::new_line(
            ctx,
            &[Vec2::new(0., -15.), Vec2::new(0., 15.)],
            2.,
            Color::from_rgb(165, 82, 76),
        )?;
        canvas.draw(&mouse_line, graphics::DrawParam::from(ctx.mouse.position()));
        let mouse_line = graphics::Mesh::new_line(
            ctx,
            &[Vec2::new(-15., 0.), Vec2::new(15., 0.)],
            2.,
            Color::from_rgb(165, 82, 76),
        )?;
        canvas.draw(&mouse_line, graphics::DrawParam::from(ctx.mouse.position()));

        canvas.finish(ctx)?;

        Ok(())
    }
}
