use std::fmt::{Display, Formatter};

pub struct Mouse {
    pub x: f32,
    pub y: f32,
}

impl Mouse {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Display for Mouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
