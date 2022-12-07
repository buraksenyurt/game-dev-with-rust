use crate::entity::asset::Asset;
use crate::entity::asset_type::AssetType;
use macroquad::prelude::{rand, screen_width, Vec2};
use macroquad::window::screen_height;

pub async fn create_cloud() -> Asset {
    let x = rand::gen_range(0., screen_width());
    let y = rand::gen_range(0., screen_height() * 0.5);
    let mut asset = Asset::new(AssetType::Cloud, Vec2::new(x, -y)).await;
    asset.velocity = Vec2::new(0., 1.);
    asset
}
