use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;

pub struct TextureManager<'a> {
    textures: HashMap<&'a str, Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let mut textures = HashMap::new();

        textures.insert(
            "player",
            texture_creator.load_texture("assets/player.png").unwrap(),
        );
        textures.insert(
            "wall",
            texture_creator.load_texture("assets/wall.png").unwrap(),
        );
        textures.insert(
            "tile",
            texture_creator.load_texture("assets/tile.png").unwrap(),
        );
        textures.insert(
            "exit_door",
            texture_creator
                .load_texture("assets/exit_door.png")
                .unwrap(),
        );
        textures.insert(
            "question_tower",
            texture_creator
                .load_texture("assets/question_tower.png")
                .unwrap(),
        );
        textures.insert(
            "ghost",
            texture_creator.load_texture("assets/snake.png").unwrap(),
        );
        textures.insert(
            "stone_tile",
            texture_creator
                .load_texture("assets/stone_tile.png")
                .unwrap(),
        );
        textures.insert(
            "ufo_1",
            texture_creator.load_texture("assets/owl.png").unwrap(),
        );
        textures.insert(
            "ufo_2",
            texture_creator.load_texture("assets/hippo.png").unwrap(),
        );
        textures.insert(
            "ufo_3",
            texture_creator.load_texture("assets/giraffe.png").unwrap(),
        );

        Self { textures }
    }

    pub fn get(&self, key: &str) -> &Texture {
        self.textures.get(key).expect("Texture not found!")
    }
}
