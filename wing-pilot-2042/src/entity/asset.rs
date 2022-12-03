use crate::entity::asset_type::AssetType;
use macroquad::prelude::{draw_texture, load_texture, Vec2, WHITE};
use macroquad::texture::Texture2D;

pub struct Asset {
    pub asset_type: AssetType,
    pub location: Vec2,
    pub texture: Texture2D,
}

impl Asset {
    pub async fn new(asset_type: AssetType, location: Vec2) -> Self {
        let texture = match asset_type {
            AssetType::Fuel => load_texture("resources/fuel_station.png").await.unwrap(),
            AssetType::GreenLand => load_texture("resources/greenland.png").await.unwrap(),
            AssetType::Island => load_texture("resources/island.png").await.unwrap(),
            AssetType::ExtraBullet => load_texture("resources/ammo_station.png").await.unwrap(),
            AssetType::Cloud => load_texture("resources/cloud.png").await.unwrap(),
        };

        Self {
            asset_type,
            location,
            texture,
        }
    }

    pub fn draw(&self) {
        draw_texture(self.texture, self.location.x, self.location.y, WHITE);
    }
}
