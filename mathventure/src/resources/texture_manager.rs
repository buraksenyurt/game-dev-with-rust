use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;

pub struct TextureManager<'a> {
    textures: HashMap<&'a str, Texture<'a>>,
    texture_creator: &'a TextureCreator<WindowContext>,
}

impl<'a> TextureManager<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            textures: HashMap::new(),
            texture_creator,
        }
    }
    pub fn add(&mut self, key: &'a str, path: &str) {
        let texture = self
            .texture_creator
            .load_texture(path)
            .expect("Failed to load texture");
        self.textures.insert(key, texture);
    }

    pub fn get(&self, key: &str) -> &Texture {
        self.textures.get(key).expect("Texture not found!")
    }
}
