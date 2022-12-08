use crate::entity::asset::Asset;
use crate::entity::asset_type::AssetType;
use macroquad::prelude::{info, rand, screen_width, Vec2};
use macroquad::window::screen_height;

pub async fn create_clouds(cloud_count: usize) -> Vec<Asset> {
    let mut clouds: Vec<Asset> = Vec::new();
    for _ in 0..cloud_count {
        let mut cloud = Asset::new(AssetType::Cloud, Vec2::new(0., 0.)).await;
        cloud.velocity = Vec2::new(0., 1.);
        let x = rand::gen_range(
            0. + cloud.texture.width() * 0.5,
            screen_width() - cloud.texture.width() * 0.5,
        );
        let y = rand::gen_range(-100., -screen_height() * 0.5);
        cloud.location.x = x;
        cloud.location.y = y;
        clouds.push(cloud);
    }
    clouds
}

pub async fn create_extra_ammo() -> Asset {
    let mut ammo = Asset::new(AssetType::ExtraAmmo, Vec2::default()).await;
    ammo.velocity = Vec2::new(0., 1.);
    let x = rand::gen_range(
        0. + ammo.texture.width(),
        screen_width() - ammo.texture.width(),
    );
    let y = rand::gen_range(ammo.texture.height(), ammo.texture.height() * 3.0);
    ammo.location.x = x;
    ammo.location.y = -y;
    ammo.lift_of_time = Some(rand::gen_range(100, 500));
    info!("Ammo lift of time {}", ammo.lift_of_time.unwrap());
    ammo
}
