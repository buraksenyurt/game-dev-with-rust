use crate::lib::constant::TRACE_TICKNESS;
use crate::{MAX_LIFT_OFF_TIME, MISSILE_LENGTH};
use macroquad::prelude::collections::storage::get;
use macroquad::prelude::*;
use std::f32::consts::PI;
use std::fmt::{Display, Formatter};

pub struct Missile {
    start_position: Vec2,
    pub position: Vec2,
    pub direction: Vec2,
    angle: f32,
    pub is_alive: bool,
    pub lift_off_time: i32,
}

impl Missile {
    pub fn produce() -> Self {
        // Füze için rastgele bir x noktası al.
        // Ekranın sağ ve sol taraflarından 4te 1 kırpılmış bir alan kullanılmakta.
        let x = rand::gen_range(
            screen_width() * 0.25,
            screen_width() - screen_width() * 0.25,
        );

        // Olası max sol açı değerini hesaplıyoruz.
        // Karşı Kenar / Komşu Kenar ın arc tanjant değeri
        let left_angle = (screen_height() / x).atan();
        // Burada da olası maksimum sağ açı değerini buluyoruz.
        let right_angle = (screen_height() / screen_width() - x).atan();
        // Belli bir değer aralığında rastgele füze açısı çekiyoruz (Radyan cinsinden)
        let angle: f32 = rand::gen_range(left_angle, PI - (left_angle + right_angle));

        Self {
            start_position: Vec2::new(x, 0.),
            position: Vec2::new(x, 0.),
            direction: Vec2::new(angle.cos(), 1.),
            angle,
            is_alive: true,
            lift_off_time: rand::gen_range(get_fps() as i32, get_fps() as i32 + MAX_LIFT_OFF_TIME),
        }
    }

    pub fn draw(&self) {
        draw_line(
            self.start_position.x,
            self.start_position.y,
            self.position.x,
            self.position.y,
            TRACE_TICKNESS,
            WHITE,
        );
        draw_rectangle(
            self.position.x,
            self.position.y,
            MISSILE_LENGTH,
            MISSILE_LENGTH,
            WHITE,
        );
    }
}

impl Display for Missile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pos: {}, Dir: {}, Angle:{} rad,{} deg, lift off {}",
            self.position,
            self.direction,
            self.angle,
            self.angle.to_degrees(),
            self.lift_off_time
        )
    }
}
