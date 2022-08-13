use bevy::prelude::{Handle, Image, TextureAtlas};

pub struct WinSize {
    pub width: f32,
    pub height: f32,
}

pub struct GameTextures {
    pub player: Handle<Image>,
    pub player_laser: Handle<Image>,
    pub enemy: Handle<Image>,
    pub enemy_laser: Handle<Image>,
    pub explosion: Handle<TextureAtlas>,
}

pub struct EnemyCount(pub u32);
