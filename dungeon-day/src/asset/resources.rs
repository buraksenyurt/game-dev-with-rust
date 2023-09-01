use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AssetList(pub Vec<HandleUntyped>);
