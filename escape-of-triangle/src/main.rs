mod components;
mod constants;
mod systems;

use bevy::prelude::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_towers, spawn_camera, spawn_player))
        .add_systems(
            Update,
            (
                handle_player_rotations,
                move_forward_player.after(handle_player_rotations),
                wrap_around,
            ),
        )
        .run();
}
