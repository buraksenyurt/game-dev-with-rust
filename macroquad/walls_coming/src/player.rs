use crate::constant::*;
use macroquad::prelude::*;

pub struct Player {
    pub rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5 - PLAYER_BOX_SIZE.x * 0.5,
                screen_height() - 30.,
                PLAYER_BOX_SIZE.x,
                PLAYER_BOX_SIZE.y,
            ),
        }
    }

    // Mouse hareketlerine göre oyuncunun ekranda sağa veya sola
    // gidişinin kontrol edildiği fonksiyondur.
    // oyuncuların makinelerindeki fps farklılıklarını ortadan kaldırmak için
    // delta time değeri kullanır
    pub fn update(&mut self, delta_time: f32) {
        // A veya D tuşlarına basılması halinde
        // -1 veya +1 mesafe belirlenir
        let mut movement = 0.;
        if is_key_down(KeyCode::A) {
            movement -= 1.;
        }
        if is_key_down(KeyCode::D) {
            movement += 1.;
        }
        // Mesafe değeri ile delta time ve oyuncu için belirlenen hız çarpılır
        // oyuncu bu değer kadar x ekseninde sağa veya sola hareket eder
        self.rect.x += movement * delta_time * PLAYER_SPEED;

        if self.rect.x < 0. {
            // Eğer sol sınıra gelindiyse orada durması sağlanır
            self.rect.x = 0.;
        }
        // Eğer ekranın sağ sınırına gelindiyse daha fazla ileri gitmemesi sağlanır
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, SKYBLUE);
    }
}
