use crate::game::star::resources::*;
use crate::game::star::systems::*;
use crate::game::PlayGroundState;
use crate::AppState;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;
pub const STAR_SIZE: f32 = 30.;
pub const NUMBER_OF_STARS: usize = 8;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_star_after_time_finished)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(PlayGroundState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_stars);
    }
}
