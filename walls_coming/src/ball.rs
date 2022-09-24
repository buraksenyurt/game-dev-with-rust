use crate::constant::{BALL_SIZE, BALL_SPEED};
use macroquad::prelude::*;

// Oyunda oyuncunun tuğlaları vurmak için kullandığı topun veri yapısı
pub struct Ball {
    pub rect: Rect,
    // Bir harekete sahip olacağından büyüklük ve yön gibi bilgileri sahip bir vektör kullanıyoruz
    pub velocity: Vec2,
}

impl Ball {
    pub fn new(position: Vec2) -> Self {
        Self {
            rect: Rect::new(position.x, position.y, BALL_SIZE, BALL_SIZE),
            // Rastgele bir hareket yönü belirleniyor
            velocity: vec2(rand::gen_range(-1., 1.), 1.).normalize(),
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, MAGENTA);
    }
    pub fn update(&mut self, delta_time: f32, bonus_speed: f32) {
        // konum güncellemeleri. velocity değerlerine göre x,y
        // değerleri belli oranda artırılır. Bu topa bir hareket efekti verecektir.
        self.rect.x += self.velocity.x * delta_time * (BALL_SPEED + bonus_speed);
        self.rect.y += self.velocity.y * delta_time * (BALL_SPEED + bonus_speed);
        // Ekran sınırlarına çarpma kontrolü.
        // x ekseninde sol veya sağ kenarlar dışına çıkıldıysa geri dönecek
        if self.rect.x < 0. {
            //Sol taraf kontrolü
            self.velocity.x = 1.;
        }
        if self.rect.x > screen_width() - self.rect.w {
            // sağ taraf kontrolü
            self.velocity.x = -1.;
        }
        // Ekran üst kısmı kontrolü. Üst kısma çarparsa geriye dönecek.
        if self.rect.y < 0. {
            self.velocity.y = 1.;
        }
    }
}
