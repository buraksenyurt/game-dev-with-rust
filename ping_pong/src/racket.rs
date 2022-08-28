pub struct Racket {
    pub height: f32,
    pub width: f32,
}

impl Racket {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
