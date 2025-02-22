mod components;
mod constants;
mod systems;

use crate::systems::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                apply_gravity_system,
                player_movement_system,
                update_position_system,
            ),
        )
        .run();
}
