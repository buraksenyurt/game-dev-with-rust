use crate::game::live_data::resources::LiveData;
use crate::game::ui::hud::components::{FuelText, MeteorInfoText, ScoreText};
use bevy::prelude::*;

pub fn update_score_text(
    mut text_query: Query<&mut Text, With<ScoreText>>,
    live_data: Res<LiveData>,
) {
    if live_data.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!(
                "{}/{}",
                live_data.exploded_meteors_count.to_string(),
                live_data.missing_meteors_count.to_string()
            );
        }
    }
}
pub fn update_last_meteor_strength_text(
    mut text_query: Query<&mut Text, With<MeteorInfoText>>,
    live_data: Res<LiveData>,
) {
    if live_data.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value =
                format!("Strength : {}", live_data.last_meteor_strength.to_string());
        }
    }
}

pub fn update_fuel_level_text(
    mut text_query: Query<&mut Text, With<FuelText>>,
    live_data: Res<LiveData>,
) {
    if live_data.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", live_data.spaceship_fuel_level.to_string());
        }
    }
}
