use bevy::prelude::*;
use bevy::sprite::TextureAtlas;

pub const TILE_SIZE: f32 = 32.;
pub const TILE_Z: f32 = 0.;
pub const PART_Z: f32 = 10.;
pub const PART_SPEED: f32 = 10.;
pub const POSITION_TOLERANCE: f32 = 0.1;

#[derive(Resource)]
pub struct GraphicsAssets {
    pub texture: Handle<TextureAtlas>,
}
