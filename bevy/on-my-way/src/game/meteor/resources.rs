use super::METEOR_SPAWN_TIME;
use bevy::prelude::*;

#[derive(Resource)]
pub struct MeteorSpawnTimer {
    pub timer: Timer,
}
impl Default for MeteorSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(METEOR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
