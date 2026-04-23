use bevy::prelude::*;
#[derive(Message)]
pub struct GameOverEvent {
    pub current_score: u32,
}
