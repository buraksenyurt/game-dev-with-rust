use bevy::prelude::{Handle, Image};

pub struct WinSize {
    pub width: f32,
    pub height: f32,
}

pub struct GameTextures {
    pub player: Handle<Image>,
}
