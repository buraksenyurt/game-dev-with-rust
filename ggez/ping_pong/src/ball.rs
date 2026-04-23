use ggez::mint::Point2;

pub struct Ball {
    pub position: Point2<f32>,
    pub velocity: Point2<f32>,
}

impl Ball {
    pub fn new(position: Point2<f32>, velocity: Point2<f32>) -> Self {
        Self { position, velocity }
    }
}
