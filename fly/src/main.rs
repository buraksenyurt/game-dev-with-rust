mod components;
mod constants;
mod game_play;
mod systems;

use crate::systems::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                update_player_state,
                player_movement_system,
                update_player_animation,
                apply_animation,
            ),
        )
        //.add_systems(Update, (spawn_flying_boxes_system, move_boxes_system))
        // .insert_resource(BoxSpawningTimer(Timer::from_seconds(
        //     SPAWN_DURATION,
        //     TimerMode::Repeating,
        // )))
        .run();
}
