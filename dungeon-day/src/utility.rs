#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}
impl Location {
    pub const UP: Location = Location { x: 0, y: 1 };
    pub const DOWN: Location = Location { x: 0, y: -1 };
    pub const LEFT: Location = Location { x: -1, y: 0 };
    pub const RIGHT: Location = Location { x: 1, y: 0 };
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
