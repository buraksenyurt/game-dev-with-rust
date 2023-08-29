use bevy::prelude::*;
pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;

use crate::events::GameOver;
use crate::game::player::PlayerPlugin;
use crate::game::score::ScorePlugin;
use crate::game::systems::{pause_game, resume_game, toggle_playground};
use crate::AppState;
use enemy::EnemyPlugin;
use star::StarPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayGroundState>()
            // Events
            .add_event::<GameOver>()
            // Plugins
            .add_plugins((PlayerPlugin, EnemyPlugin, ScorePlugin, StarPlugin))
            .add_systems(OnEnter(AppState::Game), pause_game)
            .add_systems(Update, toggle_playground.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), resume_game);
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum PlayGroundState {
    #[default]
    Running,
    Paused,
}
