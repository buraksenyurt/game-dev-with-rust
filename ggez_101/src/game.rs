use crate::color_factory::{get_colors, Color};
use crate::constant::{TINY_RECT_HEIGHT, TINY_RECT_WIDTH};
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{draw, DrawParam};
use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use std::path::Path;
use std::time::Duration;

pub struct Game {
    pub stopped: bool,
    pub rnd: ThreadRng,
    pub player: graphics::Image,
}

impl Game {
    pub fn new(context: &mut Context) -> Self {
        Game {
            stopped: false,
            rnd: thread_rng(),
            player: graphics::Image::new(context, Path::new("/senzoface2.png"))
                .expect("oyuncu karakteri yüklenemedi"),
        }
    }
}

impl EventHandler for Game {
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Down {
            self.stopped = true;
        } else if keycode == KeyCode::Up {
            self.stopped = false;
        }
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.stopped {
            let screen = graphics::drawable_size(&ctx);
            graphics::clear(ctx, graphics::Color::BLACK);
            let colors = get_colors();
            for c in colors {
                let origin = Point2 {
                    x: self.rnd.gen_range(0.0..screen.0 - TINY_RECT_WIDTH),
                    y: self.rnd.gen_range(0.0..screen.1 / 4.),
                };

                draw_rectangle(ctx, &c, origin)?;
            }
            draw_textbox(ctx)?;
            draw_image(ctx, &self.player)?;
            graphics::present(ctx)?;
            ggez::timer::sleep(Duration::from_secs_f32(0.3));
        }
        Ok(())
    }
}

fn draw_rectangle(ctx: &mut Context, color: &Color, origin: Point2<f32>) -> GameResult<()> {
    // draw vertical rectangle
    let column = graphics::Rect::new(0., 0., TINY_RECT_WIDTH, TINY_RECT_HEIGHT);
    let column_mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        column,
        graphics::Color::from_rgb(color.red, color.green, color.blue),
    )?;

    draw(
        ctx,
        &column_mesh,
        DrawParam::new().dest(Point2 {
            x: origin.x,
            y: origin.y,
        }),
    )?;

    Ok(())
}

fn draw_textbox(ctx: &mut Context) -> GameResult {
    let text_box = graphics::Text::new("For stop press Down. For restart press Up.");
    draw(
        ctx,
        &text_box,
        DrawParam::new().dest(Point2 { x: 0., y: 0. }),
    )?;
    Ok(())
}

fn draw_image(ctx: &mut Context, image: &graphics::Image) -> GameResult {
    let screen = graphics::drawable_size(&ctx);

    let screen_center_x = screen.0 * 0.5 - image.dimensions().w * 0.5;
    let screen_center_y = screen.1 * 0.5 - image.dimensions().h * 0.5;

    draw(
        ctx,
        image,
        DrawParam::new().dest(Point2 {
            x: screen_center_x,
            y: screen_center_y,
        }),
    )
}
