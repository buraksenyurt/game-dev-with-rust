use crate::game::spaceship::resources::*;
use crate::game::spaceship::systems::*;
use crate::AppState;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub const SPACESHIP_001_WIDTH: f32 = 80.;
pub const SPACESHIP_001_HEIGHT: f32 = 106.;
pub const SPACESHIP_001_SPEED: f32 = 500.;
pub const FUEL_DECREASE_PERIOD: f32 = 5.;
pub const MISSILE_003_SPEED: f32 = 350.;
pub const MISSILE_003_LAUNCHABLE_POINT: f32 = 1.;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FuelCheckTimer>()
            .init_resource::<MissileLaunchCheckTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_spaceship)
            .add_systems(
                Update,
                (
                    move_spaceship,
                    check_outside_of_the_bounds,
                    count_fuel_tick,
                    detect_collision_with_meteors,
                    decrease_spaceship_fuel,
                    detect_connected_with_fuel_station,
                    fire_missile,
                    count_launch_time,
                )
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), despawn_spaceship);
    }
}
