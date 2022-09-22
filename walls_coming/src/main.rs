mod ball;
mod block;
mod builder;
mod collider;
mod constant;
mod player;

use crate::ball::Ball;
use crate::block::BlockType;
use crate::builder::create_blocks;
use crate::collider::in_collision;
use crate::player::Player;
use macroquad::prelude::*;

#[macroquad::main("WallsComing")]
async fn main() {
    // Oyuncu nesnesi oluşturulur
    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::new();
    let mut game_score = 0;
    let mut player_lives = 3;

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
        // Çaprışma kontrolünün yapıldığı kısım
        for ball in balls.iter_mut() {
            // Oyuncu dikdörtgeni ile topun çarpışıp çarpışmadığına bakılıyor
            in_collision(&mut ball.rect, &player.rect, &mut ball.velocity);
            // Sahnedeki tüm bloklar dolaşılıyor
            for block in blocks.iter_mut() {
                //ve topun çarptığı bir bloksa
                if in_collision(&mut ball.rect, &block.rect, &mut ball.velocity) {
                    // bloğun strength değeri 1 azalıyor. 0 olanlar sahneden çıkarılacaklar
                    block.strength -= 1;
                    // Blok yok ediliyorsa oyuncunun puanını bloğun tipine göre artırıyoruz
                    if block.strength <= 0 {
                        match block.block_type {
                            BlockType::Brick => game_score += 1,
                            BlockType::Stone => game_score += 3,
                            BlockType::Iron => game_score += 5,
                        }
                    }
                }
            }
        }
        // oyunucun canlarını hesapladığımız ve gerekirse azalttığımız kısım
        // Top oyuncuyu geçerse sahadan çıkarıyoruz
        let balls_count = balls.len();
        // Topun oyuncuyu geçtiğini ekran yüksekliğine göre anlayabiliriz
        balls.retain(|ball| ball.rect.y < screen_height());
        let removed_balls_count = balls_count - balls.len();
        //ve ayrıca sahada hiç top kalmadıysa(birden fazla top olabileceği senaryosuna göre çalışıyor sistem)
        // oyunucunun puanını bir azaltıyoruz
        if removed_balls_count > 0 {
            player_lives -= 1;
        }

        // Gücü 0dan büyük olanları tutmamızı sağlar. Böylece oyunucunun topla vurduklarından
        // gücü sıfıra inmiş olanlar sahneden çıkarılırlar.
        blocks.retain(|b| b.strength > 0);

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

        draw_score_box(&mut game_score, &mut player_lives);
        // Bir sonraki frame için beklenir
        next_frame().await
    }
}

fn draw_score_box(game_score: &mut i32, player_lives: &mut i32) {
    // Skor kutucuğunun çizildiği kısım
    let score_box = format!("Skor:{}. Kalan Can {} ", game_score, player_lives);
    let score_box_dimension = measure_text(&score_box, None, 24, 1.);
    draw_text_ex(
        &score_box,
        screen_width() * 0.5 - score_box_dimension.width * 0.5,
        20.,
        TextParams {
            color: BLACK,
            font_size: 24,
            ..Default::default()
        },
    );
}
