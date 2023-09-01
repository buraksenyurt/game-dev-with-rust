use bevy::prelude::*;

#[derive(States, Clone, Debug, Hash, Eq, PartialEq,Default)]
pub enum AppState {
    #[default]
    LoadAssets,
    Game,
}
