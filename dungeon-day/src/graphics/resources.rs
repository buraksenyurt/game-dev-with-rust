use bevy::prelude::*;
use bevy::sprite::TextureAtlas;

pub const TILE_SIZE: f32 = 32.;

#[derive(Resource)]
pub struct GraphicsAssets {
    pub texture: Handle<TextureAtlas>,
}
