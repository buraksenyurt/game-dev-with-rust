fn main() {
    println!("Hello, world!");
}

struct Position {
    x: f32,
    y: f32,
}

struct Size {
    width: f32,
    height: f32,
}

// struct Velocity {
//     x: f32,
//     y: f32,
// }

struct SmartSquare {
    pos: Position,
    size: Size,
    color: [f32; 4],
}

impl SmartSquare {
    fn new(pos: Position, size: Size, color: [f32; 4]) -> Self {
        Self { pos, size, color }
    }

    fn free_fall(&mut self, y_speed: f32) {
        self.pos.y += y_speed;
    }

    fn check_y_range(&self, window_height: f32) -> bool {
        self.pos.y > window_height
    }
}
