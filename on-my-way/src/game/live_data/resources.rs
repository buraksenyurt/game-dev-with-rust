use super::*;
#[derive(Resource, Debug)]
pub struct LiveData {
    pub current_meteor_count: u8,
    pub spaceship_fuel_level: f32,
    pub missing_meteors_count: u32,
    pub exploded_meteors_count: u32,
    pub last_meteor_strength: u8,
}
impl Default for LiveData {
    fn default() -> Self {
        Self {
            current_meteor_count: 0,
            spaceship_fuel_level: DEFAULT_FUEL_LEVEL,
            missing_meteors_count: 0,
            exploded_meteors_count: 0,
            last_meteor_strength: 0,
        }
    }
}
