use crate::{Vec2, CITY_HEIGHT, TURRET_MULTIPLIER};
use macroquad::prelude::{mouse_position, DARKGREEN};
use macroquad::shapes::draw_line;
use macroquad::window::{screen_height, screen_width};

pub struct Turret {
    location: Vec2,
}

impl Turret {
    pub fn new() -> Self {
        Self {
            location: Vec2::new(screen_width() * 0.5, screen_height() - CITY_HEIGHT),
        }
    }

    pub fn draw(&self) {
        // Güncel mouse pozisyonu alınır
        let m = mouse_position();
        // Uçaksavar merkez noktası ile mouse'un olduğu nokta arasındaki vektör bulunur
        let v = Vec2::new(m.0 - self.location.x, m.1 - self.location.y);
        // Aradaki vektör birim vektöre dönüştürülür
        let unit_v = v.normalize();
        // ve sabit bir skaler değer ile çarpılır. Böylece uzunluğu değiştirilir.
        let add_v = unit_v * TURRET_MULTIPLIER;
        // Yeni elde edilen noktaya doğru bir çizgi çizdirilir
        draw_line(
            self.location.x,
            self.location.y,
            self.location.x + add_v.x,
            self.location.y + add_v.y,
            3.,
            DARKGREEN,
        )
    }
}
