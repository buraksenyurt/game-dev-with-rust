use bevy::prelude::*;

#[derive(States, Clone, Debug, Hash, Eq, PartialEq, Default)]
pub enum AppState {
    #[default]
    LoadAssets,
    Game,
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq, SystemSet)]
pub enum GameState {
    #[default]
    None,
    PlayerInput,
    TurnUpdate,
}
