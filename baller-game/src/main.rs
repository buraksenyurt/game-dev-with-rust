mod common;
pub mod enemy;
mod events;
mod player;
mod score;
mod star;
mod systems;

use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;
use crate::score::ScorePlugin;
use crate::star::StarPlugin;
use bevy::prelude::*;
use events::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PlayerPlugin, EnemyPlugin, ScorePlugin, StarPlugin))
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (refresh_everything, exit_game, handle_game_over))
        .run();
}
