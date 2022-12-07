use crate::entity::asset_type::AssetType;
use macroquad::prelude::{draw_texture, load_texture, Vec2, WHITE};
use macroquad::rand;
use macroquad::texture::Texture2D;

#[derive(Copy, Clone)]
pub struct Asset {
    pub asset_type: AssetType,
    pub location: Vec2,
    pub texture: Texture2D,
    pub on_stage: bool,
    pub velocity: Vec2,
}

impl Asset {
    pub async fn new(asset_type: AssetType, location: Vec2) -> Self {
        let cloud_names = vec![
            "resources/cloud1.png",
            "resources/cloud2.png",
            "resources/cloud3.png",
            "resources/cloud4.png",
        ];
        let texture = match asset_type {
            AssetType::Fuel => load_texture("resources/fuel_station.png").await.unwrap(),
            AssetType::GreenLand => load_texture("resources/greenland.png").await.unwrap(),
            AssetType::Island => load_texture("resources/island.png").await.unwrap(),
            AssetType::ExtraBullet => load_texture("resources/ammo_station.png").await.unwrap(),
            AssetType::Cloud => {
                let index = rand::gen_range(0, 4);
                load_texture(cloud_names[index]).await.unwrap()
            }
        };

        Self {
            asset_type,
            location,
            texture,
            on_stage: true,
            velocity: Vec2::default(),
        }
    }

    pub fn draw(&self) {
        draw_texture(self.texture, self.location.x, self.location.y, WHITE);
    }
}
