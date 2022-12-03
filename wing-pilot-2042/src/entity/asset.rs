use macroquad::prelude::Vec2;
use crate::entity::asset_type::AssetType;

pub struct Asset {
    pub asset_type: AssetType,
    pub location:Vec2
}
