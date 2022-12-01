#[derive(Clone, Copy)]
pub struct Stage {
    pub level: usize,
    pub max_missile_count: i32,
    pub total_missile_count: i32,
    pub missile_speed_factor: f32,
    pub max_bullet_count: usize,
}

impl Stage {
    pub fn new(
        level: usize,
        max_missile_count: i32,
        total_missile_count: i32,
        missile_speed_factor: f32,
        max_bullet_count: usize,
    ) -> Self {
        Self {
            level,
            max_missile_count,
            total_missile_count,
            missile_speed_factor,
            max_bullet_count,
        }
    }
}
