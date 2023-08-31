use crate::game::missile::systems::*;
use crate::AppState;
use bevy::prelude::*;

pub mod components;
mod systems;
pub struct MissilePlugin;

impl Plugin for MissilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_missile,
                detect_collision_with_meteors,
                claim_hitted,
                check_outside_of_the_bounds,
                animate_explosion_sprites,
                despawn_explosions,
            )
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(OnExit(AppState::Game), despawn_missiles);
    }
}
