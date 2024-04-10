use crate::entity::BlockType;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;

pub struct TextureManager<'a> {
    textures: HashMap<BlockType, Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let mut textures = HashMap::new();

        textures.insert(
            BlockType::Player,
            texture_creator.load_texture("assets/player.png").unwrap(),
        );
        textures.insert(
            BlockType::Wall,
            texture_creator.load_texture("assets/wall.png").unwrap(),
        );
        textures.insert(
            BlockType::Tile,
            texture_creator.load_texture("assets/tile.png").unwrap(),
        );
        textures.insert(
            BlockType::ExitDoor,
            texture_creator
                .load_texture("assets/exit_door.png")
                .unwrap(),
        );
        textures.insert(
            BlockType::QuestionTower,
            texture_creator
                .load_texture("assets/question_tower.png")
                .unwrap(),
        );
        textures.insert(
            BlockType::Ghost,
            texture_creator.load_texture("assets/snake.png").unwrap(),
        );
        textures.insert(
            BlockType::StoneTile,
            texture_creator
                .load_texture("assets/stone_tile.png")
                .unwrap(),
        );
        textures.insert(
            BlockType::Ufo,
            texture_creator.load_texture("assets/owl.png").unwrap(),
        );

        Self { textures }
    }

    pub fn get_texture(&self, block_type: &BlockType) -> &Texture {
        self.textures.get(block_type).expect("Texture not found!")
    }
}
