mod block;
mod builder;
mod constant;
mod player;

use crate::builder::create_blocks;
use crate::player::Player;
use macroquad::prelude::*;

#[macroquad::main("WallsComing")]
async fn main() {
    // Oyuncu nesnesi oluşturulur
    let mut player = Player::new();
    let mut blocks = Vec::new();
    create_blocks(&mut blocks);
    // Oyun döngümüz
    loop {
        player.update(get_frame_time());
        // Ekran temizleni ve zemin beyaz renk yapılır
        clear_background(WHITE);
        // Oyuncu nesnesi ekrana çizili
        player.draw();
        // Duvarları çizdiriyoruz
        for b in blocks.iter() {
            b.draw();
        }
        // Bir sonraki frame için beklenir
        next_frame().await
    }
}
