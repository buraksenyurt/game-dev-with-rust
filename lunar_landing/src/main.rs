use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureQuery, WindowCanvas};
use sdl2::ttf;
use sdl2::video::Window;
use std::cmp;
use std::time::Duration;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut velocity = Vector::new(0., 0.);
    let mut shuttle = Shuttle::new(Point::new(200, 80), 10000);

    let window = video_subsystem
        .window("Lunar Landing 2049", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let game = init_game(WIDTH, HEIGHT);

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    velocity.x -= 0.50;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    velocity.x += 0.50;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    velocity.y += 1.50;
                    shuttle.fuel_level -= 2;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    velocity.y -= 1.;
                    shuttle.fuel_level -= 10;
                }
                _ => {}
            }
        }

        //draw_landing_area(&mut canvas)?;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        draw_game_area(&mut canvas, &game)?;
        draw_landing_gear(&mut canvas, &game)?;
        //draw_mountain(&mut canvas)?;
        let v_point = velocity.to_point();
        shuttle.draw(&mut canvas, Color::RGB(255, 255, 0), v_point)?;
        velocity.y += 0.05;
        shuttle.fuel_level -= 1;

        draw_text(
            &mut canvas,
            &ttf_context,
            &format!("Fuel: {}", shuttle.fuel_level),
            14,
            Color::RGBA(255, 255, 255, 255),
            WIDTH - 100,
            10,
        )?;
        draw_text(
            &mut canvas,
            &ttf_context,
            &format!(
                "({}:{})",
                shuttle.position.x + v_point.x,
                shuttle.position.y + v_point.y
            ),
            14,
            Color::RGBA(255, 255, 255, 255),
            WIDTH - 100,
            30,
        )?;

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn draw_game_area(canvas: &mut WindowCanvas, game: &Game) -> Result<(), String> {
    for i in 0..game.mountain_points.len() - 1 {
        let start = game.mountain_points[i];
        let end = game.mountain_points[i + 1];
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_line(start, end)?;
    }

    Ok(())
}

fn draw_landing_gear(canvas: &mut WindowCanvas, game: &Game) -> Result<(), String> {
    for &(platform_start, platform_end, leg_start, leg_end) in &game.landing_areas {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_line(platform_start, platform_end)?;
        canvas.draw_line(platform_start, leg_start)?;
        canvas.draw_line(platform_end, leg_end)?;
    }

    Ok(())
}

pub struct Shuttle {
    pub position: Point,
    pub fuel_level: i32,
}

impl Shuttle {
    pub fn new(position: Point, fuel_level: i32) -> Self {
        Self {
            position,
            fuel_level,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        color: Color,
        velocity: Point,
    ) -> Result<(), String> {
        let point = self.position;

        canvas.set_draw_color(color);
        canvas.draw_rect(Rect::new(
            point.x + velocity.x,
            point.y + velocity.y,
            25,
            25,
        ))?;
        canvas.draw_line(
            Point::new(point.x + velocity.x, point.y + 25 + velocity.y),
            Point::new(point.x - 10 + velocity.x, point.y + 55 + velocity.y),
        )?;
        canvas.draw_line(
            Point::new(point.x + 25 + velocity.x, point.y + 25 + velocity.y),
            Point::new(point.x + 35 + velocity.x, point.y + 55 + velocity.y),
        )?;

        Ok(())
    }
}
pub struct Vector {
    pub x: f32,
    pub y: f32,
}
impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn to_point(&self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }
}

fn draw_text(
    canvas: &mut Canvas<Window>,
    ttf_context: &ttf::Sdl2TtfContext,
    text: &str,
    font_size: u16,
    color: Color,
    x: i32,
    y: i32,
) -> Result<(), String> {
    let font = ttf_context.load_font(   "fonts/OpenSans-Bold.ttf", font_size)?;
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = texture.query();
    canvas.copy(&texture, None, Some(Rect::new(x, y, width, height)))?;
    Ok(())
}

pub fn draw_strong_line(
    canvas: &mut Canvas<Window>,
    start: Point,
    end: Point,
    color: Color,
    thickness: i32,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    for i in 0..thickness {
        let offset = i - thickness / 2;
        canvas.draw_line(
            Point::new(start.x + offset, start.y + offset),
            Point::new(end.x + offset, end.y + offset),
        )?;
    }
    Ok(())
}

struct Game {
    mountain_points: Vec<Point>,
    landing_areas: Vec<(Point, Point, Point, Point)>,
}

fn init_game(width: i32, height: i32) -> Game {
    let mut rng = rand::thread_rng();
    let ground_level = height - 50;

    let mut mountain_points = Vec::new();
    let mut x = 0;
    while x < width {
        let peak_height = rng.gen_range(ground_level - 150..ground_level);
        mountain_points.push(Point::new(x, peak_height));
        x += rng.gen_range(80..120);
    }
    mountain_points.push(Point::new(
        width,
        rng.gen_range(ground_level - 150..ground_level),
    ));

    let mut landing_areas = Vec::new();
    while landing_areas.len() < 2 {
        let landing_start = rng.gen_range(0..width - 50);
        let landing_end = cmp::min(landing_start + 60, width);
        let platform_height = rng.gen_range(ground_level - 150..ground_level - 100);
        let left_leg = find_ground_height(&mountain_points, landing_start);
        let right_leg = find_ground_height(&mountain_points, landing_end);

        landing_areas.push((
            Point::new(landing_start, platform_height),
            Point::new(landing_end, platform_height),
            Point::new(landing_start, left_leg),
            Point::new(landing_end, right_leg),
        ));
    }

    Game {
        mountain_points,
        landing_areas,
    }
}

fn find_ground_height(points: &[Point], x: i32) -> i32 {
    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];
        if x >= p1.x && x <= p2.x {
            return interpolate_height(&[p1, p2], x);
        }
    }
    0
}

fn interpolate_height(points: &[Point], x: i32) -> i32 {
    if x <= points.first().unwrap().x {
        return points.first().unwrap().y;
    }
    if x >= points.last().unwrap().x {
        return points.last().unwrap().y;
    }

    for window in points.windows(2) {
        let (p1, p2) = (window[0], window[1]);
        if x >= p1.x && x <= p2.x {
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            return p1.y + (x - p1.x) * dy / dx;
        }
    }
    points.first().unwrap().y
}
