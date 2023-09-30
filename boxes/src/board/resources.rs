use crate::components::Vector;
use bevy::prelude::*;
use std::collections::HashMap;
#[derive(Default, Resource)]
pub struct ActiveBoard {
    pub tiles: HashMap<Vector, Entity>,
}

#[derive(Resource)]
pub struct GraphicsAssets {
    pub texture: Handle<TextureAtlas>,
}

#[derive(Resource, Default)]
pub struct AssetList(pub Vec<HandleUntyped>);
