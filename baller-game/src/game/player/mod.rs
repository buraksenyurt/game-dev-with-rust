use crate::game::player::systems::*;
use crate::game::PlayGroundState;
use crate::AppState;
use bevy::prelude::*;

pub mod components;
mod systems;
pub const PLAYER_SIZE: f32 = 64.;
pub const PLAYER_SPEED: f32 = 500.;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(
                Update,
                (
                    check_enemy_hit_player,
                    control_player_movement,
                    check_player_movement.after(control_player_movement),
                    check_player_hits_star,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(PlayGroundState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
