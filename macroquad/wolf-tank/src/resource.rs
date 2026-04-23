use macroquad::prelude::{load_texture, Texture2D};

pub enum TextureType {
    Tank,
    Garrison,
    Bullet,
}

pub async fn get_texture(texture_type: TextureType) -> Texture2D {
    match texture_type {
        TextureType::Tank => load_texture(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/tank.png")).await.unwrap(),
        TextureType::Garrison => load_texture(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/garrison.png")).await.unwrap(),
        TextureType::Bullet => load_texture(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/bullet.png")).await.unwrap(),
    }
}
