use crate::board::components::*;
use crate::components::Vector;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default, Resource)]
pub struct ActiveBoard {
    pub tiles: HashMap<Vector, (Entity, TileKind)>,
}

#[derive(Resource)]
pub struct GraphicsAssets {
    pub texture: Handle<TextureAtlas>,
}

#[derive(Resource, Default)]
pub struct AssetList(pub Vec<HandleUntyped>);
