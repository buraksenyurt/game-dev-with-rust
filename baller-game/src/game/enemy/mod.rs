use crate::game::enemy::resources::EnemySpawnTimer;
use crate::game::enemy::systems::*;
use crate::game::PlayGroundState;
use crate::AppState;
use bevy::app::App;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;
pub const ENEMY_SIZE: f32 = 64.;
pub const ENEMEY_SPEED: f32 = 200.;
pub const NUMBER_OF_ENEMIES: usize = 6;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    enemy_out_of_bound_check.after(enemy_movement),
                    check_enemy_movement,
                    enemy_spawn_timer,
                    spawn_enemy_after_time_finished,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(PlayGroundState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}
