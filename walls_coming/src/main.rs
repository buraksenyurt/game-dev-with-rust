mod constant;
mod player;

use crate::player::Player;
use macroquad::prelude::*;

#[macroquad::main("WallsComing")]
async fn main() {
    // Oyuncu nesnesi oluşturulur
    let mut player = Player::new();
    // Oyun döngümüz
    loop {
        player.update(get_frame_time());
        // Ekran temizleni ve zemin beyaz renk yapılır
        clear_background(WHITE);
        // Oyuncu nesnesi ekrana çizili
        player.draw();
        // Bir sonraki frame için beklenir
        next_frame().await
    }
}
