pub enum GameState {
    Main,
    Playing(Stage),
    Dead,
    Win,
}

#[derive(Clone, Copy)]
pub struct Stage {
    pub level: u32,
    pub max_missile_count: i32,
    pub total_missile_count: i32,
    pub missile_speed_factor: f32,
}

impl Stage {
    pub fn new(
        level: u32,
        max_missile_count: i32,
        total_missile_count: i32,
        missile_speed_factor: f32,
    ) -> Self {
        Self {
            level,
            max_missile_count,
            total_missile_count,
            missile_speed_factor,
        }
    }
}
