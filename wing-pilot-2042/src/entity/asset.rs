use crate::entity::asset_type::AssetType;
use macroquad::prelude::{draw_texture, load_texture, Rect, Vec2, WHITE};
use macroquad::rand;
use macroquad::texture::Texture2D;

#[derive(Copy, Clone, PartialEq)]
pub struct Asset {
    pub asset_type: AssetType,
    pub location: Vec2,
    pub texture: Texture2D,
    pub on_stage: bool,
    pub velocity: Vec2,
    pub lift_of_time: Option<i32>,
}

impl Asset {
    pub async fn new(asset_type: AssetType, location: Vec2) -> Self {
        let clouds = vec![
            "resources/cloud1.png",
            "resources/cloud2.png",
            "resources/cloud3.png",
            "resources/cloud4.png",
            "resources/cloud5.png",
        ];
        let grounds = vec![
            "resources/ground_1.png",
            "resources/ground_2.png",
            "resources/ground_3.png",
        ];
        let texture = match asset_type {
            AssetType::Ground => {
                let index = rand::gen_range(0, grounds.len());
                load_texture(grounds[index]).await.unwrap()
            }
            AssetType::ExtraAmmo => load_texture("resources/extra_ammo.png").await.unwrap(),
            AssetType::Cloud => {
                let index = rand::gen_range(0, clouds.len());
                load_texture(clouds[index]).await.unwrap()
            }
            AssetType::None => Texture2D::empty(),
        };

        Self {
            asset_type,
            location,
            texture,
            on_stage: true,
            velocity: Vec2::default(),
            lift_of_time: None,
        }
    }

    pub fn draw(&self) {
        draw_texture(self.texture, self.location.x, self.location.y, WHITE);
    }

    pub async fn get_rect(&self, scale: f32) -> Rect {
        Rect::new(
            self.location.x - (self.location.x * scale),
            self.location.y - (self.location.y * scale),
            self.texture.width() + (self.texture.width() * scale),
            self.texture.height() + (self.texture.height() * scale),
        )
    }
}

impl Default for Asset {
    fn default() -> Self {
        Self {
            asset_type: AssetType::None,
            location: Vec2::default(),
            texture: Texture2D::empty(),
            on_stage: false,
            velocity: Vec2::default(),
            lift_of_time: None,
        }
    }
}
