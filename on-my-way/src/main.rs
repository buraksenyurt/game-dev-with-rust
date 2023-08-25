mod game;
mod meteor;
mod spaceship;
mod systems;

use crate::game::GamePlugin;
use crate::meteor::MeteorPlugin;
use crate::spaceship::SpaceshipPlugin;
use crate::systems::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((GamePlugin, MeteorPlugin, SpaceshipPlugin))
        .add_systems(Startup, (spawn_camera_system, spawn_stars_system))
        .run();
}
