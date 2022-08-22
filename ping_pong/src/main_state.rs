use crate::constants::{
    Direction, BALL_SIZE, BALL_SIZE_HALF, BALL_SPEED, CENTER_LINE_WIDTH, PADDING, PLAYER_SPEED,
    RACKET_H, RACKET_H_HALF, RACKET_W, RACKET_W_HALF,
};
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{draw, DrawParam};
use ggez::input::keyboard;
use ggez::mint::Point2;
use ggez::timer::delta;
use ggez::{graphics, Context, GameResult};
use rand::{thread_rng, Rng};

pub struct MainState {
    p1_position: Point2<f32>,
    p2_position: Point2<f32>,
    ball_position: Point2<f32>,
    ball_velocity: Point2<f32>,
    center_line_position: Point2<f32>,
    p1_score: u32,
    p2_score: u32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (scr_width, scr_height) = graphics::drawable_size(ctx);
        //println!("{}X{}", scr_width, scr_height);
        let (scr_width_half, scr_height_half) = (scr_width * 0.5, scr_height * 0.5);
        let mut ball_point = Point2 { x: 0., y: 0. };
        get_rand_position(&mut ball_point, BALL_SPEED, BALL_SPEED);
        MainState {
            p1_position: Point2 {
                x: RACKET_W_HALF + PADDING,
                y: scr_height_half,
            },
            p2_position: Point2 {
                x: scr_width - RACKET_W_HALF - PADDING,
                y: scr_height_half,
            },
            ball_position: Point2 {
                x: scr_width_half,
                y: scr_height_half,
            },
            ball_velocity: ball_point,
            center_line_position: Point2 {
                x: scr_width_half,
                y: 0.,
            },
            p1_score: 0,
            p2_score: 0,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let delta_time = delta(ctx).as_secs_f32();
        let (screen_width, screen_height) = graphics::drawable_size(ctx);

        move_to(&mut self.p2_position, Direction::Up, KeyCode::Up, ctx);
        move_to(&mut self.p2_position, Direction::Down, KeyCode::Down, ctx);
        move_to(&mut self.p1_position, Direction::Up, KeyCode::W, ctx);
        move_to(&mut self.p1_position, Direction::Down, KeyCode::S, ctx);

        self.ball_position.x += self.ball_velocity.x * delta_time;
        self.ball_position.y += self.ball_velocity.y * delta_time;

        if self.ball_position.x < 0. {
            self.ball_position.x = screen_width * 0.5;
            self.ball_position.y = screen_height * 0.5;
            get_rand_position(&mut self.ball_velocity, BALL_SPEED, BALL_SPEED);
            self.p2_score += 1;
            //println!("P1 -> {}, P2 -> {}", self.p1_score, self.p2_score);
        }

        if self.ball_position.x > screen_width {
            self.ball_position.x = screen_width * 0.5;
            self.ball_position.y = screen_height * 0.5;
            get_rand_position(&mut self.ball_velocity, BALL_SPEED, BALL_SPEED);
            self.p1_score += 1;
            //println!("P1 -> {}, P2 -> {}", self.p1_score, self.p2_score);
        }

        if self.ball_position.y < BALL_SIZE_HALF {
            self.ball_position.y = BALL_SIZE_HALF;
            self.ball_velocity.y = self.ball_velocity.y.abs();
        } else if self.ball_position.y > screen_height - BALL_SIZE_HALF {
            self.ball_position.y = screen_height - BALL_SIZE_HALF;
            self.ball_velocity.y = -self.ball_velocity.y.abs();
        }

        if is_player_catch_the_ball(self.p1_position, self.ball_position) {
            self.ball_velocity.x = self.ball_velocity.x.abs();
        }

        if is_player_catch_the_ball(self.p2_position, self.ball_position) {
            self.ball_velocity.x = -self.ball_velocity.x.abs();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from_rgb(55, 109, 93));

        draw_center_line(ctx, self)?;
        draw_racket(ctx, self.p1_position)?;
        draw_racket(ctx, self.p2_position)?;
        draw_ball(ctx, self.ball_position)?;
        draw_score_box(ctx, self)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn is_player_catch_the_ball(player_position: Point2<f32>, ball_position: Point2<f32>) -> bool {
    let result = ball_position.x - BALL_SIZE_HALF < player_position.x + RACKET_W_HALF
        && ball_position.x + BALL_SIZE_HALF > player_position.x - RACKET_W_HALF
        && ball_position.y - BALL_SIZE_HALF < player_position.y + RACKET_H_HALF
        && ball_position.y + BALL_SIZE_HALF > player_position.y - RACKET_H_HALF;

    result
}

fn draw_score_box(ctx: &mut Context, main_state: &MainState) -> GameResult {
    let screen_width = graphics::drawable_size(ctx).0;
    let screen_width_half = screen_width * 0.5;

    let score_box = graphics::Text::new(format!(
        "Oyuncu 1 :{} vs Oyuncu 2 :{}",
        main_state.p1_score, main_state.p2_score
    ));

    let mut score_position = Point2 {
        x: screen_width_half,
        y: 25.,
    };
    let score_box_dimension = score_box.dimensions(ctx);
    score_position.x -= score_box_dimension.w as f32 * 0.5;
    score_position.y -= score_box_dimension.h as f32 * 0.5;

    draw(ctx, &score_box, DrawParam::new().dest(score_position))?;

    Ok(())
}

fn draw_ball(ctx: &mut Context, position: Point2<f32>) -> GameResult<()> {
    let ball = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
    let ball_mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        ball,
        graphics::Color::WHITE,
    )?;

    draw(ctx, &ball_mesh, DrawParam::new().dest(position))?;

    Ok(())
}

fn draw_racket(ctx: &mut Context, position: Point2<f32>) -> GameResult<()> {
    let racket = graphics::Rect::new(-RACKET_W_HALF, -RACKET_H_HALF, RACKET_W, RACKET_H);
    let racket_mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        racket,
        graphics::Color::WHITE,
    )?;

    draw(ctx, &racket_mesh, DrawParam::new().dest(position))?;

    Ok(())
}

fn draw_center_line(ctx: &mut Context, main_state: &MainState) -> GameResult<()> {
    let screen_height = graphics::drawable_size(ctx).1;
    let center_line = graphics::Rect::new(
        -CENTER_LINE_WIDTH * 0.5,
        0.,
        CENTER_LINE_WIDTH,
        screen_height,
    );
    let center_line_mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        center_line,
        graphics::Color::WHITE,
    )?;

    draw(
        ctx,
        &center_line_mesh,
        DrawParam::new().dest(main_state.center_line_position),
    )?;

    Ok(())
}

fn move_to(
    player_position: &mut Point2<f32>,
    direction: Direction,
    key_code: KeyCode,
    ctx: &mut Context,
) {
    let screen_height = graphics::drawable_size(ctx).1;
    let delta_time = delta(ctx).as_secs_f32();

    if keyboard::is_key_pressed(ctx, key_code) {
        player_position.y += f32::from(direction) * PLAYER_SPEED * delta_time;
    }
    border_check(
        &mut player_position.y,
        RACKET_H_HALF,
        screen_height - RACKET_H_HALF,
    );
}

fn border_check(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn get_rand_position(point: &mut Point2<f32>, x: f32, y: f32) {
    let mut rnd = thread_rng();
    point.x = match rnd.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    point.y = match rnd.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}
