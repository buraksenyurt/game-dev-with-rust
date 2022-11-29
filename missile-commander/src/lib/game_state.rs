pub enum GameState {
    Main,
    Playing(Level),
    Dead,
    Win,
}

#[derive(Clone, Copy)]
pub struct Level {
    pub difficulty: u32,
    pub max_missile_count: i32,
    pub total_missile_count: i32,
    pub missile_speed_factor: f32,
}

impl Level {
    pub fn new(
        difficulty: u32,
        max_missile_count: i32,
        total_missile_count: i32,
        missile_speed_factor: f32,
    ) -> Self {
        Self {
            difficulty,
            max_missile_count,
            total_missile_count,
            missile_speed_factor,
        }
    }
}
