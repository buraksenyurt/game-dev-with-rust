use crate::score::resources::*;
use crate::score::systems::*;
use bevy::prelude::*;

pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_systems(
                Update,
                (update_score, update_high_score, handle_high_scores_updated),
            );
    }
}
