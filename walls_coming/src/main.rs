mod ball;
mod block;
mod builder;
mod collider;
mod constant;
mod game_state;
mod player;
mod texturer;

use crate::ball::Ball;
use crate::block::{BlockType, Powerup};
use crate::builder::create_blocks;
use crate::collider::in_collision;
use crate::constant::{CAPTAIN_SLOW_LIFETIME, CAPTAIN_SLOW_SPEED, DEFAULT_ELONGATION, PLAYER_BOX_SIZE};
use crate::game_state::GameState;
use crate::player::Player;
use crate::texturer::{draw_score_box, draw_title_text};
use macroquad::prelude::*;

#[macroquad::main("WallsComing")]
async fn main() {
    let mut game_state = GameState::Menu;

    // Oyuncu nesnesi oluşturulur
    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::new();
    let mut game_score = 0;
    let mut player_lives = 3;
    let mut extra_speed: f32 = 0.;
    let mut in_extra_speed = false;
    let mut extra_speed_counter = 0;

    // Bloklar üretilir
    create_blocks(&mut blocks);
    // Ekranın tam orta yerinde konumlanacak şekilde bir top üretilir
    balls.push(Ball::new(vec2(screen_width() * 0.5, screen_height() * 0.5)));

    // Oyun döngümüz
    loop {
        match game_state {
            GameState::Menu => {
                // Oyun Menu durumunda iken oyuncu SPACE tuşuna basarsa
                // oyunun durumu Playing'e geçer. Buna göre match ifadelerindeki
                // playing kısmı çalışır
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
            }
            GameState::Playing => {
                // Oyun oynanıyor(Playing) moddayken çalışan kısımdır

                // Topun hızını yavaşlatan powerup devrede iken bir sayaç kullanıyoruz
                // Böylece powerup'ın sadece belli süre etkin kalmasını sağlamaktayız
                if in_extra_speed {
                    extra_speed_counter += 1;
                    if extra_speed_counter >= CAPTAIN_SLOW_LIFETIME  {
                        in_extra_speed = false;
                        extra_speed=0.;
                    }
                }

                // Oyuncu pozisyonu için güncelleme çağrılır.
                player.update(get_frame_time());
                // topların pozisyonları için güncelleme çağrılır.
                for ball in balls.iter_mut() {
                    ball.update(get_frame_time(), extra_speed);
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
                                match &block.block_type {
                                    BlockType::Brick => game_score += 1,
                                    BlockType::Stone => game_score += 3,
                                    BlockType::Iron => game_score += 5,
                                    // Eğer bir powerup gelirse ona göre işlem yapıyoruz
                                    // Mesela boyu kısaltan bir tuğlaya denk gelirse oyuncu
                                    // bloğunun boyu kısalıyor gibi
                                    BlockType::Bonus(p) => match p {
                                        Powerup::SpudWebb => {
                                            player.rect.w = PLAYER_BOX_SIZE.x - DEFAULT_ELONGATION
                                        }
                                        Powerup::YaoMing => {
                                            player.rect.w = PLAYER_BOX_SIZE.x + DEFAULT_ELONGATION
                                        }
                                        Powerup::CaptainSlow => {
                                            extra_speed = -CAPTAIN_SLOW_SPEED;
                                            in_extra_speed = true
                                        }
                                    },
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
                    balls.push(Ball::new(vec2(screen_width() * 0.5, screen_height() * 0.5)));
                    if player_lives <= 0 {
                        game_state = GameState::PlayerDead;
                    }
                }

                // Gücü 0dan büyük olanları tutmamızı sağlar. Böylece oyunucunun topla vurduklarından
                // gücü sıfıra inmiş olanlar sahneden çıkarılırlar.
                blocks.retain(|b| b.strength > 0);
                // Oyuncu tüm blokları temizlemişse oyun durumu seviye tamamlandıya çekilir
                if blocks.is_empty() {
                    game_state = GameState::LevelCompleted;
                }
            }
            GameState::LevelCompleted | GameState::PlayerDead => {
                // Tüm bloklar temizlendiğinde veya oyuncunun tüm canları bittiğinde
                // yeniden space tuşuna basıp oyuna başlanabilir.
                if is_key_pressed(KeyCode::Space) {
                    // Bu durumda oyun nesneleri de sıfırlanır
                    game_state = GameState::Menu;
                    player = Player::new();
                    game_score = 0;
                    player_lives = 3;
                    extra_speed = 0.;
                    in_extra_speed = false;
                    balls.clear();
                    balls.push(Ball::new(vec2(screen_width() * 0.5, screen_height() * 0.5)));
                    blocks.clear();
                    create_blocks(&mut blocks);
                }
            }
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

        // Oyunun içinde bulunduğu duruma göre
        match game_state {
            GameState::Menu => {
                // Menu durumunda iken başlamak için SPACE tuşuna basınız yazdırıyoruz
                draw_title_text("Baslamak icin SPACE tusuna bas");
            }
            GameState::Playing => {
                draw_score_box(&mut game_score, &mut player_lives);
            }
            GameState::LevelCompleted => {
                draw_title_text(&format!(
                    "{} canda KAZANDIN! Skorun {}",
                    player_lives, game_score
                ));
            }
            GameState::PlayerDead => {
                draw_title_text(&format!("Skorun {} ama kaybettin :(", game_score));
            }
        }

        // Bir sonraki frame için beklenir
        next_frame().await
    }
}
