use bevy::prelude::*;
#[derive(Resource, Debug)]
pub struct Score {
    pub value: u32,
}
impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}
#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> Self {
        Self { scores: Vec::new() }
    }
}
