use bevy::prelude::*;

#[derive(Resource)]
pub struct WinSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
    pub player_laser: Handle<Image>,
    pub enemy: Handle<Image>,
    pub enemy_laser: Handle<Image>,
    pub explosion: Handle<Image>,
    pub explosion_layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct EnemyCount(pub u32);
