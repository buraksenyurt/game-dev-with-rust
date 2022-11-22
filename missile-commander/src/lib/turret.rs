use crate::{CITY_HEIGHT, TURRET_MULTIPLIER, TURRET_THICKNESS};
use macroquad::prelude::{mouse_position, Vec2, DARKGREEN};
use macroquad::shapes::draw_line;
use macroquad::window::{screen_height, screen_width};

pub struct Turret {
    location: Vec2,
    pub muzzle_point: Vec2,
}

impl Turret {
    pub fn new() -> Self {
        Self {
            location: Vec2::new(screen_width() * 0.5, screen_height() - CITY_HEIGHT),
            muzzle_point: Vec2::default(),
        }
    }

    pub fn draw(&mut self) {
        // Güncel mouse pozisyonu alınır
        let m = mouse_position();
        // Uçaksavar merkez noktası ile mouse'un olduğu nokta arasındaki vektör bulunur
        let v = Vec2::new(m.0 - self.location.x, m.1 - self.location.y);
        // Aradaki vektör birim vektöre dönüştürülür
        let unit_v = v.normalize();
        // ve sabit bir skaler değer ile çarpılır. Böylece uzunluğu değiştirilir.
        let add_v = unit_v * TURRET_MULTIPLIER;
        // namlu noktası bulunur
        let mp = Vec2::new(self.location.x + add_v.x, self.location.y + add_v.y);
        self.muzzle_point = mp;
        // ilk konumdan namlu noktasına doğru bir çizgi çizdirilir
        draw_line(
            self.location.x,
            self.location.y,
            mp.x,
            mp.y,
            TURRET_THICKNESS,
            DARKGREEN,
        );
    }
}
