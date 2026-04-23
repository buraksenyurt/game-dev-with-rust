use bevy::prelude::*;
#[derive(Event)]
pub struct GameOverEvent {
    pub current_score: u32,
}
