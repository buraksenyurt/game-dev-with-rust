use crate::block::Block;
use crate::constant::{BLOCK_PADDING, BLOCK_SIZE};
use crate::{vec2, BlockType, Powerup};
use macroquad::prelude::*;

// Ekranın üst kısmına 6X6 boyutlarında aralarında boşluklar da bulunan blokları veren fonksiyon
pub fn create_blocks(blocks: &mut Vec<Block>) {
    let (width, height) = (6, 5);
    // Blok büyüklüğünü hesaplarken padding değerlerini de hesaba katıyoruz
    let total_block_size = BLOCK_SIZE + vec2(BLOCK_PADDING, BLOCK_PADDING);
    // ekranda blokların dizilmeye başladıyacağı x,y koordinatlarını belirliyoruz
    let start_position = vec2(
        (screen_width() - (total_block_size.x * width as f32)) * 0.5,
        50.,
    );
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(start_position + vec2(block_x, block_y)));
    }

    let power_up_index: usize = rand::gen_range(0, blocks.len());
    blocks[power_up_index].block_type = BlockType::Bonus(Powerup::Tall);

    let power_up_index: usize = rand::gen_range(0, blocks.len());
    blocks[power_up_index].block_type = BlockType::Bonus(Powerup::Short);
}
