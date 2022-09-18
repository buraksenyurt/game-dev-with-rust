mod ball;
mod block;
mod builder;
mod constant;
mod player;

use crate::ball::Ball;
use crate::builder::create_blocks;
use crate::player::Player;
use macroquad::prelude::*;

#[macroquad::main("WallsComing")]
async fn main() {
    // Oyuncu nesnesi oluşturulur
    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::new();

    // Bloklar üretilir
    create_blocks(&mut blocks);
    // Ekranın tam orta yerinde konumlanacak şekilde bir top üretilir
    balls.push(Ball::new(vec2(screen_width() * 0.5, screen_height() * 0.5)));

    // Oyun döngümüz
    loop {
        // Oyuncu pozisyonu için güncelleme çağrılır.
        player.update(get_frame_time());
        // topların pozisyonları için güncelleme çağrılır.
        for ball in balls.iter_mut() {
            ball.update(get_frame_time());
        }
        // Ekran temizleni ve zemin beyaz renk yapılır
        clear_background(WHITE);
        // Oyuncu nesnesi ekrana çizili
        player.draw();
        // Duvarları çizdiriyoruz
        for block in blocks.iter() {
            block.draw();
        }
        // Oyunda olan top nesnelerini çizdiriyoruz
        for ball in balls.iter() {
            ball.draw();
        }
        // Bir sonraki frame için beklenir
        next_frame().await
    }
}
