use ggez::mint::Point2;

pub struct Player {
    pub id: u8,
    pub position: Point2<f32>,
    pub score: u32,
}

impl Player {
    pub fn new(id: u8, position: Point2<f32>, score: u32) -> Self {
        Self {
            id,
            position,
            score,
        }
    }
}
