pub struct Score {
    pub total_point: u32,
    pub lives: u8,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            lives: 3,
            total_point: 0,
        }
    }
}
