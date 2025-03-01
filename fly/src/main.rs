mod components;
mod constants;
mod game_play;
mod systems;

use crate::components::BoxSpawningTimer;
use crate::constants::SPAWN_DURATION;
use crate::systems::*;
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                apply_gravity_system,
                player_movement_system,
                update_player_position_system,
                apply_animation,
            ),
        )
        //.add_systems(Update, (spawn_flying_boxes_system, move_boxes_system))
        .insert_resource(BoxSpawningTimer(Timer::from_seconds(
            SPAWN_DURATION,
            TimerMode::Repeating,
        )))
        .run();
}
