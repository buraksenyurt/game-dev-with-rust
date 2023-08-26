use crate::events::GameOver;
use crate::game::score::resources::*;
use bevy::prelude::*;

// Bu sistem de GameOver event'lerini dinler.
// Dolayısıyla bir event birden fazla sistem tarafından okunabilir.
pub fn update_high_score(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.iter() {
        high_scores
            .scores
            .push(("Player".to_string(), event.final_score));
    }
}

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn handle_high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        info!("High scores changed to {:?}", high_scores);
    }
}
pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        info!("Current score {}", score.value.to_string());
    }
}
