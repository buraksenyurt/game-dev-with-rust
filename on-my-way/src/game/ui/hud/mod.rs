use crate::game::ui::hud::systems::layout::*;
use crate::game::ui::hud::systems::updates::*;
use crate::AppState;
use bevy::prelude::*;

mod components;
mod styles;
mod systems;

pub struct HeadUpDisplayPlugin;

impl Plugin for HeadUpDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_head_up_display)
            .add_systems(
                Update,
                (
                    update_score_text,
                    update_fuel_level_text,
                    update_last_meteor_strength_text,
                )
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), despawn_head_up_display);
    }
}
