use crate::constants::{
    Direction, BALL_SIZE, BALL_SIZE_HALF, BALL_SPEED, CENTER_LINE_WIDTH, P1_BONUS_IMAGE,
    P2_BONUS_IMAGE, PADDING, PLAYER_SPEED, RACKET_H, RACKET_H_HALF, RACKET_W, RACKET_W_HALF,
};
use crate::player::Player;
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{draw, DrawParam, Font, PxScale};
use ggez::input::keyboard;
use ggez::mint::Point2;
use ggez::timer::delta;
use ggez::{graphics, Context, GameResult};
use rand::{thread_rng, Rng};
use std::path::Path;

pub struct MainState {
    player_1: Player,
    player_2: Player,
    ball_position: Point2<f32>,
    ball_velocity: Point2<f32>,
    p1_bonus_position: Point2<f32>,
    p2_bonus_position: Point2<f32>,
    bonus_velocity: Point2<f32>,
    center_line_position: Point2<f32>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (scr_width, scr_height) = graphics::drawable_size(ctx);
        //println!("{}X{}", scr_width, scr_height);
        let (scr_width_half, scr_height_half) = (scr_width * 0.5, scr_height * 0.5);
        let mut ball_point = Point2 { x: 0., y: 0. };
        get_rand_position(&mut ball_point, BALL_SPEED, BALL_SPEED);

        let rand_bonus_point = get_rand_position_on_center_line(&ctx);
        MainState {
            player_1: Player::new(
                1,
                Point2 {
                    x: RACKET_W_HALF + PADDING,
                    y: scr_height_half,
                },
                0,
            ),
            player_2: Player::new(
                2,
                Point2 {
                    x: scr_width - RACKET_W_HALF - PADDING,
                    y: scr_height_half,
                },
                0,
            ),
            ball_position: Point2 {
                x: scr_width_half,
                y: scr_height_half,
            },
            ball_velocity: ball_point,
            center_line_position: Point2 {
                x: scr_width_half,
                y: 0.,
            },
            p1_bonus_position: rand_bonus_point,
            p2_bonus_position: rand_bonus_point,
            bonus_velocity: ball_point,
        }
    }

    fn check_borderline(&mut self, screen_width: f32, screen_height: f32) {
        if self.ball_position.x < 0. {
            self.ball_position.x = screen_width * 0.5;
            self.ball_position.y = screen_height * 0.5;
            get_rand_position(&mut self.ball_velocity, BALL_SPEED, BALL_SPEED);
            self.player_2.score += 1;
            //println!("P1 -> {}, P2 -> {}", self.p1_score, self.p2_score);
        }

        if self.ball_position.x > screen_width {
            self.ball_position.x = screen_width * 0.5;
            self.ball_position.y = screen_height * 0.5;
            get_rand_position(&mut self.ball_velocity, BALL_SPEED, BALL_SPEED);
            self.player_1.score += 1;
            //println!("P1 -> {}, P2 -> {}", self.p1_score, self.p2_score);
        }

        if self.ball_position.y < BALL_SIZE_HALF {
            self.ball_position.y = BALL_SIZE_HALF;
            self.ball_velocity.y = self.ball_velocity.y.abs();
        } else if self.ball_position.y > screen_height - BALL_SIZE_HALF {
            self.ball_position.y = screen_height - BALL_SIZE_HALF;
            self.ball_velocity.y = -self.ball_velocity.y.abs();
        }

        if is_player_catch_the_ball(self.player_1.position, self.ball_position) {
            self.ball_velocity.x = self.ball_velocity.x.abs();
        }

        if is_player_catch_the_ball(self.player_2.position, self.ball_position) {
            self.ball_velocity.x = -self.ball_velocity.x.abs();
        }
    }

    fn bonus_control(&mut self, ctx: &&mut Context, delta_time: f32, screen_width: f32) {
        self.p1_bonus_position.x -= self.bonus_velocity.x * delta_time * 0.5;
        self.p2_bonus_position.x += self.bonus_velocity.x * delta_time * 0.5;

        if self.p1_bonus_position.x < 0. {
            self.p1_bonus_position.x = screen_width * 0.5;
            self.p1_bonus_position.y = 0.;
            self.p1_bonus_position = get_rand_position_on_center_line(&ctx);
        }

        if self.p2_bonus_position.x >= screen_width {
            self.p2_bonus_position.x = screen_width * 0.5;
            self.p2_bonus_position.y = 0.;
            self.p2_bonus_position = get_rand_position_on_center_line(&ctx);
        }

        if is_player_catch_the_bonus(self.player_1.position, self.p1_bonus_position) {
            println!("Player 1 bonusu yakaladı");
            self.player_1.score += 2;
        }

        if is_player_catch_the_bonus(self.player_2.position, self.p2_bonus_position) {
            println!("Player 2 bonusu yakaladı");
            self.player_2.score += 2;
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let delta_time = delta(ctx).as_secs_f32();
        let (screen_width, screen_height) = graphics::drawable_size(ctx);

        move_to(&mut self.player_2.position, Direction::Up, KeyCode::Up, ctx);
        move_to(
            &mut self.player_2.position,
            Direction::Down,
            KeyCode::Down,
            ctx,
        );
        move_to(&mut self.player_1.position, Direction::Up, KeyCode::W, ctx);
        move_to(
            &mut self.player_1.position,
            Direction::Down,
            KeyCode::S,
            ctx,
        );

        self.ball_position.x += self.ball_velocity.x * delta_time;
        self.ball_position.y += self.ball_velocity.y * delta_time;
        self.check_borderline(screen_width, screen_height);

        self.bonus_control(&ctx, delta_time, screen_width);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from_rgb(55, 109, 93));

        draw_center_line(ctx, self)?;
        draw_racket(ctx, self.player_1.position)?;
        draw_racket(ctx, self.player_2.position)?;
        draw_ball(ctx, self.ball_position)?;
        draw_score_box(ctx, self)?;
        draw_bonus(ctx, self.p1_bonus_position, P1_BONUS_IMAGE)?;
        draw_bonus(ctx, self.p2_bonus_position, P2_BONUS_IMAGE)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn is_player_catch_the_ball(player_position: Point2<f32>, ball_position: Point2<f32>) -> bool {
    let result = ball_position.x < player_position.x + RACKET_W_HALF
        && ball_position.x + BALL_SIZE > player_position.x - RACKET_W_HALF
        && ball_position.y - BALL_SIZE < player_position.y + RACKET_H_HALF
        && ball_position.y + BALL_SIZE > player_position.y - RACKET_H_HALF;

    if result {
        println!(
            "Ball Position {}x{},Player Position {}x{}",
            ball_position.x, ball_position.y, player_position.x, player_position.y
        );
    }
    result
}

fn is_player_catch_the_bonus(player_position: Point2<f32>, bonus_position: Point2<f32>) -> bool {
    let result = bonus_position.x < player_position.x + RACKET_W_HALF
        && bonus_position.x + BALL_SIZE > player_position.x - RACKET_W_HALF
        && bonus_position.y - BALL_SIZE < player_position.y + RACKET_H_HALF
        && bonus_position.y + BALL_SIZE > player_position.y - RACKET_H_HALF;

    if result {
        println!(
            "Bonus Position {}x{},Player Position {}x{}",
            bonus_position.x, bonus_position.y, player_position.x, player_position.y
        );
    }
    result
}

fn draw_score_box(ctx: &mut Context, main_state: &MainState) -> GameResult {
    let screen_width = graphics::drawable_size(ctx).0;
    let screen_width_half = screen_width * 0.5;

    let mut score_box = graphics::Text::new(format!(
        "Oyuncu L :{} vs Oyuncu R :{}",
        main_state.player_1.score, main_state.player_2.score
    ));
    score_box.set_font(Font::default(), PxScale::from(32.));

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
    // let ball = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
    // let ball_mesh = graphics::Mesh::new_rectangle(
    //     ctx,
    //     graphics::DrawMode::fill(),
    //     ball,
    //     graphics::Color::WHITE,
    // )?;
    let ball_image = graphics::Image::new(ctx, Path::new("/SoccerBall.png"))?;
    draw(ctx, &ball_image, DrawParam::new().dest(position))?;
    //draw(ctx, &ball_mesh, DrawParam::new().dest(position))?;

    Ok(())
}

fn draw_bonus(ctx: &mut Context, position: Point2<f32>, resource: &str) -> GameResult<()> {
    let apple_image = graphics::Image::new(ctx, Path::new(resource))?;
    draw(ctx, &apple_image, DrawParam::new().dest(position))?;

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

fn get_rand_position_on_center_line(ctx: &Context) -> Point2<f32> {
    let (screen_width, screen_height) = graphics::drawable_size(ctx);
    let center_of_screen = screen_width * 0.5;

    let mut rnd = thread_rng();
    let random_y: f32 = rnd.gen_range(50.0..screen_height - 50.);
    Point2 {
        x: center_of_screen,
        y: random_y,
    }
}
