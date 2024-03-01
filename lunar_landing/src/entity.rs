use crate::constants::*;
use crate::game::Game;
use crate::math::Vector;
use crate::utility::draw_text;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::video::Window;
use std::f64::consts::PI;
use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

pub struct Shuttle {
    pub position: Point,
    pub fuel_level: i32,
    pub velocity: Vector,
}

impl Shuttle {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(50..WIDTH - 50);
        let y = rng.gen_range(10..WIDTH / 8);
        let position = Point::new(x, y);
        Self {
            position,
            fuel_level: DEFAULT_FUEL_LEVEL,
            velocity: Vector::default(),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, color: Color) -> Result<(), String> {
        let point = self.position;
        let velocity = self.velocity.to_point();
        let base_x = point.x + velocity.x;
        let base_y = point.y + velocity.y;
        let leg_y = point.y + SHUTTLE_HEAD_WIDTH * 2 + velocity.y;

        canvas.set_draw_color(color);
        canvas.draw_rect(Rect::new(
            base_x,
            base_y,
            SHUTTLE_HEAD_WIDTH as u32,
            SHUTTLE_HEAD_WIDTH as u32,
        ))?;

        let half_width = SHUTTLE_HEAD_WIDTH / 2;
        let quarter_width = SHUTTLE_HEAD_WIDTH / 4;

        // Sol bacak
        let left_leg_start = Point::new(base_x, base_y + SHUTTLE_HEAD_WIDTH);
        let left_leg_end = Point::new(base_x - half_width, leg_y);
        canvas.draw_line(left_leg_start, left_leg_end)?;

        // Sol ayak
        canvas.draw_line(
            left_leg_end,
            Point::new(left_leg_end.x - quarter_width, leg_y),
        )?;

        // Sağ bacak
        let right_leg_start = Point::new(base_x + SHUTTLE_HEAD_WIDTH, base_y + SHUTTLE_HEAD_WIDTH);
        let right_leg_end = Point::new(base_x + SHUTTLE_HEAD_WIDTH + half_width, leg_y);
        canvas.draw_line(right_leg_start, right_leg_end)?;

        // Sağ ayak
        canvas.draw_line(
            right_leg_end,
            Point::new(right_leg_end.x + quarter_width, leg_y),
        )?;

        Ok(())
    }
    pub fn calculate_foot_points(&self) -> ((Point, Point), (Point, Point)) {
        let foot_width = SHUTTLE_HEAD_WIDTH / 2;
        let foot_y = self.position.y + SHUTTLE_HEAD_WIDTH * 2 + self.velocity.y as i32;
        let left_foot_start = Point::new(
            self.position.x - foot_width + self.velocity.x as i32,
            foot_y,
        );
        let left_foot_end = Point::new(self.position.x + self.velocity.x as i32, foot_y);
        let right_foot_start = Point::new(self.position.x + self.velocity.x as i32, foot_y);
        let right_foot_end = Point::new(
            self.position.x + foot_width + self.velocity.x as i32,
            foot_y,
        );
        (
            (left_foot_start, left_foot_end),
            (right_foot_start, right_foot_end),
        )
    }
    pub fn is_landed(&self, game: &Game) -> bool {
        let mut is_landed = false;
        for lp in &game.landing_platforms {
            // println!("{:?} {:?}", lp.p1, lp.p2);
            if lp.check_collision(self) {
                // println!("Congrats!!! Shuttle has been landed...");
                is_landed = true;
                break;
            }
        }
        is_landed
    }

    pub fn is_fuel_critical(&self) -> bool {
        self.fuel_level <= DEFAULT_FUEL_LEVEL / 4
    }

    pub fn is_low_altitude(&self) -> bool {
        self.position.y + self.velocity.y as i32 >= HEIGHT - HEIGHT / 4
    }

    pub fn check_collision_with_meteor(&self, meteor: &Meteor) -> bool {
        let body = Rect::new(
            self.position.x + self.velocity.x as i32,
            self.position.y + self.velocity.y as i32,
            SHUTTLE_HEAD_WIDTH as u32,
            (SHUTTLE_HEAD_WIDTH * 2) as u32,
        );
        // println!("{:?}", body);
        let x = meteor.center.x + meteor.velocity.x as i32 + meteor.radius as i32;
        let y = meteor.center.y + meteor.velocity.y as i32 + meteor.radius as i32;
        let meteor_front_point = Point::new(x, y);

        // println!("{:?}", meteor_front_point);

        body.contains_point(meteor_front_point)
    }
}

#[derive(PartialEq)]
pub struct LandingPlatform {
    pub p1: Point,
    pub p2: Point,
    pub left_leg: Point,
    pub right_leg: Point,
}

impl LandingPlatform {
    pub fn new(p1: Point, p2: Point, left_leg: Point, right_leg: Point) -> Self {
        Self {
            p1,
            p2,
            left_leg,
            right_leg,
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_line(self.p1, self.p2)?;
        canvas.draw_line(self.p1, self.left_leg)?;
        canvas.draw_line(self.p2, self.right_leg)?;

        Ok(())
    }
    fn get_top_edge(&self) -> (Point, Point) {
        (self.p1, self.p2)
    }
    pub fn check_collision(&self, shuttle: &Shuttle) -> bool {
        let ((shuttle_left_foot, _), (shuttle_right_foot, _)) = shuttle.calculate_foot_points();
        // println!("Shuttle:{:?}:{:?}", shuttle_left_foot, shuttle_right_foot);
        let (platform_start, platform_end) = self.get_top_edge();
        let feet_above_platform = shuttle_left_foot.y >= platform_start.y - 1
            && shuttle_right_foot.y <= platform_start.y + 1;
        let feet_within_platform =
            shuttle_left_foot.x >= platform_start.x && shuttle_right_foot.x <= platform_end.x;

        feet_above_platform && feet_within_platform
    }
}

#[derive(PartialEq)]
pub struct Meteor {
    pub center: Point,
    pub sides: u8,
    pub radius: i16,
    pub rot_angle: f64,
    pub velocity: Vector,
    pub in_range: bool,
}

impl Meteor {
    pub fn new(center: Point, sides: u8, radius: i16, rot_angle: f64, in_range: bool) -> Self {
        Self {
            center,
            sides,
            radius,
            rot_angle,
            velocity: Vector::default(),
            in_range,
        }
    }
    pub fn mark_range(&mut self) {
        let curr_x = self.center.x + self.velocity.x as i32;
        let curr_y = self.center.y + self.velocity.y as i32;
        //println!("{curr_x}:{curr_y}");
        if curr_x > WIDTH || curr_y > HEIGHT {
            self.in_range = false;
            //println!("Meteor is out of range");
        }
    }
    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let mut points = Vec::new();
        let angle_step = 2.0 * PI / self.sides as f64;
        let angle_radian = self.rot_angle * PI / 180.;
        let velocity = self.velocity.to_point();
        for i in 0..self.sides {
            let angle = angle_step * i as f64 + angle_radian;
            let x = (self.radius as f64 * angle.cos()).round() as i32 + self.center.x + velocity.x;
            let y = (self.radius as f64 * angle.sin()).round() as i32 + self.center.y + velocity.y;
            points.push((x, y));
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for i in 0..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = if i == points.len() - 1 {
                points[0]
            } else {
                points[i + 1]
            };

            canvas.draw_line(Point::new(x1, y1), Point::new(x2, y2))?;
        }

        Ok(())
    }
}

pub struct Hud {
    fuel_warn_last_blink: Instant,
    fuel_warn_blink_interval: Duration,
    fuel_warn_blink_visible: bool,
    alt_warn_last_blink: Instant,
    alt_warn_blink_interval: Duration,
    alt_warn_blink_visible: bool,
}

impl Hud {
    pub fn new() -> Self {
        Self {
            fuel_warn_last_blink: Instant::now(),
            fuel_warn_blink_visible: true,
            fuel_warn_blink_interval: Duration::from_millis(750),
            alt_warn_last_blink: Instant::now(),
            alt_warn_blink_visible: true,
            alt_warn_blink_interval: Duration::from_millis(500),
        }
    }
    pub fn draw(&mut self, shuttle: &Shuttle, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let v_point = shuttle.velocity.to_point();
        draw_text(
            canvas,
            &format!("Fuel: {}", shuttle.fuel_level),
            14,
            Color::RGBA(255, 255, 255, 255),
            WIDTH - 100,
            10,
        )?;
        draw_text(
            canvas,
            &format!(
                "Pos {}:{}",
                shuttle.position.x + v_point.x,
                shuttle.position.y + v_point.y
            ),
            14,
            Color::RGBA(255, 255, 255, 255),
            WIDTH - 100,
            30,
        )?;
        self.show_fuel_warning(shuttle, canvas)?;
        self.show_low_altitude_warning(shuttle, canvas)?;

        Ok(())
    }

    fn show_fuel_warning(
        &mut self,
        shuttle: &Shuttle,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), String> {
        if shuttle.is_fuel_critical() {
            let now = Instant::now();
            if now.duration_since(self.fuel_warn_last_blink) >= self.fuel_warn_blink_interval {
                self.fuel_warn_blink_visible = !self.fuel_warn_blink_visible;
                self.fuel_warn_last_blink = now;
            }
            if self.fuel_warn_blink_visible {
                draw_text(
                    canvas,
                    "Fuel Critical",
                    14,
                    Color::RGBA(255, 0, 0, 255),
                    30,
                    30,
                )?;
            }
        }
        Ok(())
    }

    fn show_low_altitude_warning(
        &mut self,
        shuttle: &Shuttle,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), String> {
        if shuttle.is_low_altitude() {
            let now = Instant::now();
            if now.duration_since(self.alt_warn_last_blink) >= self.alt_warn_blink_interval {
                self.alt_warn_blink_visible = !self.alt_warn_blink_visible;
                self.alt_warn_last_blink = now;
            }
            if self.alt_warn_blink_visible {
                draw_text(canvas, "Altitude", 14, Color::RGBA(255, 0, 0, 255), 30, 10)?;
            }
        }
        Ok(())
    }
}

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    OutOfFuel,
    MeteorHit,
    JobsDone,
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameState::Playing => {
                write!(f, "Playing")
            }
            GameState::OutOfFuel => {
                write!(f, "Out of fuel. Game is over!")
            }
            GameState::MeteorHit => {
                write!(f, "Meteor hit. Game is over!")
            }
            GameState::JobsDone => {
                write!(f, "Job's done... Well done!")
            }
        }
    }
}
