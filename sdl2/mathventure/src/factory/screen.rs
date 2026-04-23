use crate::factory::Dimension;

pub struct Screen {
    pub title: String,
    pub dimension: Dimension,
}

impl Screen {
    pub fn new(title: String, dimension: Dimension) -> Self {
        Self { title, dimension }
    }
}
