use crate::entity::asset_type::AssetType;
use macroquad::prelude::Vec2;

pub struct Asset {
    pub asset_type: AssetType,
    pub location: Vec2,
}
