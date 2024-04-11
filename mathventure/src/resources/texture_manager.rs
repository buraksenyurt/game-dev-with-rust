use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct AssetManager<'a> {
    textures: HashMap<String, Texture<'a>>,
}

impl<'a> AssetManager<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let mut textures = HashMap::new();
        let assets_path = Path::new("assets/");
        if assets_path.exists() && assets_path.is_dir() {
            for entry in fs::read_dir(assets_path).expect("Failed to read assets directory") {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();

                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("png") {
                    if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                        let texture_path = path.to_str().expect("Failed to convert path to str");
                        let texture = texture_creator
                            .load_texture(texture_path)
                            .unwrap_or_else(|_| panic!("Failed to load texture: {}", texture_path));
                        textures.insert(filename.to_string(), texture);
                    }
                }
            }
        }

        Self { textures }
    }

    pub fn get(&self, key: &str) -> &Texture {
        self.textures.get(key).expect("Texture not found!")
    }
}
