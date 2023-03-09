use opengl_graphics::Texture;
use std::path::Path;
use texture::TextureSettings;

pub fn calculate_square(tile_size: f64, x: i32, y: i32) -> [f64; 4] {
    [
        x as f64 * tile_size,
        y as f64 * tile_size,
        tile_size,
        tile_size,
    ]
}

// Bu fonksiyon ile assets klasöründeki imajların Texture olarak yüklenmesi sağlanıyor
pub fn load_texture(image_name: &str) -> Texture {
    Texture::from_path(
        Path::new(&format!("assets/{}.png", image_name)),
        &TextureSettings::new(),
    )
    .expect(&format!("Failed to load: {}", image_name))
}
