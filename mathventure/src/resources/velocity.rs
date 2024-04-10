pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

impl Velocity {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
