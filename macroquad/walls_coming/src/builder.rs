use crate::block::Block;
use crate::constant::{BLOCK_PADDING, BLOCK_SIZE, COLUMN_COUNT, ROW_COUNT};
use crate::{vec2, BlockType, Powerup};
use macroquad::prelude::*;
use macroquad::rand::gen_range;

// Ekranın üst kısmına 6X6 boyutlarında aralarında boşluklar da bulunan blokları veren fonksiyon
pub fn create_blocks(blocks: &mut Vec<Block>) {
    let (width, height) = (COLUMN_COUNT, ROW_COUNT);
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

    let tall_index: usize = gen_range(0, width);
    blocks[tall_index].block_type = BlockType::Bonus(Powerup::YaoMing);

    let short_index: usize = gen_range(width * 2, blocks.len());
    blocks[short_index].block_type = BlockType::Bonus(Powerup::SpudWebb);

    let captain_slow_index: usize = gen_range(height, blocks.len());
    blocks[captain_slow_index].block_type = BlockType::Bonus(Powerup::CaptainSlow);
}
