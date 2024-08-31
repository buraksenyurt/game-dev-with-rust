use crate::hud::ScoreText;
use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct Score {
    pub player_damage: i8,
    pub total_hit: u8,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            player_damage: 100,
            total_hit: 0,
        }
    }
}

pub struct GameDataPlugin;

impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<Score>(Score::default())
            .add_systems(Update, update_score_text);
    }
}

pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, live_data: Res<Score>) {
    if live_data.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}/{}", live_data.total_hit, live_data.player_damage);
        }
    }
}
