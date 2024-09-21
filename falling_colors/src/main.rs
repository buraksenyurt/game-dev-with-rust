use glfw::{fail_on_errors, Action, Context, Key, WindowMode};
use std::time::{Duration, Instant};

const SCREEN_HEIGHT: f32 = 600f32;
const SCREEN_WIDTH: f32 = 400f32;
const SQUARE_FALL_SPEED: f32 = 5f32;

fn main() {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    let (mut window, events) = glfw
        .create_window(
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
            "Falling Colors",
            WindowMode::Windowed,
        )
        .unwrap();

    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }

        window.swap_buffers();
    }
}

struct Game {
    player: Hero,
    smart_squares: Vec<SmartSquare>,
    last_fall: Instant,
}

impl Game {
    fn new() -> Self {
        Self {
            player: Hero::new(
                "Antonyo Kunte".to_string(),
                ShapeSize {
                    width: 32f32,
                    height: 32f32,
                },
                [0f32, 1f32, 1f32, 1f32],
            ),
            smart_squares: vec![],
            last_fall: Instant::now(),
        }
    }

    fn spawn_square(&mut self) {
        let square = SmartSquare::new(
            Position {
                x: rand::random::<f32>() * SCREEN_WIDTH,
                y: 0f32,
            },
            ShapeSize {
                width: 32f32,
                height: 32f32,
            },
            [rand::random(), rand::random(), rand::random(), 1f32],
        );
        self.smart_squares.push(square);
    }

    fn update(&mut self) {
        for square in &mut self.smart_squares {
            square.free_fall(SQUARE_FALL_SPEED);
        }

        self.smart_squares
            .retain(|s| !s.is_out_of_screen(SCREEN_HEIGHT));

        if self.last_fall.elapsed() >= Duration::from_secs(1) {
            self.spawn_square();
            self.last_fall = Instant::now();
        }
    }
}

struct Position {
    x: f32,
    y: f32,
}

struct ShapeSize {
    width: f32,
    height: f32,
}

// struct Velocity {
//     x: f32,
//     y: f32,
// }

struct SmartSquare {
    pos: Position,
    size: ShapeSize,
    color: [f32; 4],
}

impl SmartSquare {
    fn new(pos: Position, size: ShapeSize, color: [f32; 4]) -> Self {
        Self { pos, size, color }
    }

    fn free_fall(&mut self, y_speed: f32) {
        self.pos.y += y_speed;
    }

    fn is_out_of_screen(&self, window_height: f32) -> bool {
        self.pos.y > window_height
    }
}

struct Hero {
    name: String,
    size: ShapeSize,
    color: [f32; 4],
}

impl Hero {
    fn new(name: String, size: ShapeSize, color: [f32; 4]) -> Self {
        Self { name, size, color }
    }
}
