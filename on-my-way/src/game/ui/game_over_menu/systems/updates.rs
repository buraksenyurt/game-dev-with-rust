use bevy::prelude::*;

use crate::events::GameOverEvent;
use crate::game::ui::game_over_menu::components::FinalScoreText;

pub fn update_final_score_text(
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
) {
    for event in game_over_event_reader.iter() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("Final Score: {}", event.current_score.to_string());
        }
    }
}
