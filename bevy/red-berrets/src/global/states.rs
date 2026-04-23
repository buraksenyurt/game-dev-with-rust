use bevy::{state::state::SubStates};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, SubStates)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Play,
}
