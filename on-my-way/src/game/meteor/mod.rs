use crate::game::meteor::resources::MeteorSpawnTimer;
use crate::game::meteor::systems::*;
use crate::AppState;
use bevy::app::App;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;
pub const METEOR_SPAWN_TIME: f32 = 5.;
pub const MAX_METEOR_COUNT: u8 = 5;
pub const METEOR_ROTATE_DEGREE: f32 = 30.;
pub struct MeteorPlugin;
impl Plugin for MeteorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MeteorSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_meteors)
            .add_systems(
                Update,
                (
                    move_meteors,
                    check_outside_of_the_bounds,
                    count_meteor_spawn_tick,
                    spawn_after_time_finished,
                    claim_hitted,
                )
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), despawn_meteors);
    }
}
