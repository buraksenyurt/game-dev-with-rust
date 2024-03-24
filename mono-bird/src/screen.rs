pub struct Screen {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Screen {
    pub fn new(title: String, width: u32, height: u32) -> Self {
        Self {
            title,
            width,
            height,
        }
    }
}
