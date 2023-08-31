use bevy::prelude::*;

//use crate::events::GameOverEvent;
use crate::game::live_data::resources::LiveData;
use crate::game::ui::game_over_menu::components::FinalScoreText;

pub fn update_final_score_text(
    //mut game_over_event_reader: EventReader<GameOverEvent>,
    mut text_query: Query<&mut Text, With<FinalScoreText>>,
    live_data: Res<LiveData>,
) {
    // Event'i bir sebepten okuyamıyorum. LiveData Resource'a geçtim.
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!(
            "Score: {}\nDismissed : {}\nFuel : {}\nMissile Cost : {}",
            live_data.exploded_meteors_count,
            live_data.missing_meteors_count,
            live_data.spaceship_fuel_level,
            live_data.used_missile_count
        );
    }
}
