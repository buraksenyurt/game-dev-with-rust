mod components;
mod styles;
mod systems;

use crate::game::ui::game_over_menu::systems::layout::*;
use crate::game::ui::game_over_menu::systems::updates::update_final_score_text;
use crate::AppState;
use bevy::prelude::*;

pub struct GameOverMenuPlugin;
impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(
                Update,
                update_final_score_text.run_if(in_state(AppState::GameOver)),
            )
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}
