mod events;
mod game;
mod meteor;
mod spaceship;
mod station;
mod systems;

use crate::events::GameOverEvent;
use crate::game::GamePlugin;
use crate::meteor::MeteorPlugin;
use crate::spaceship::SpaceshipPlugin;
use crate::station::FuelStationPlugin;
use crate::systems::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((GamePlugin, MeteorPlugin, SpaceshipPlugin, FuelStationPlugin))
        .add_event::<GameOverEvent>()
        .add_systems(Startup, (spawn_camera, spawn_stars, handle_game_over))
        .run();
}
