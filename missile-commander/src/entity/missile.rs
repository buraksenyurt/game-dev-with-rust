use crate::game::constant::TRACE_TICKNESS;
use crate::{MAX_LIFT_OFF_TIME, MISSILE_LENGTH};
use macroquad::prelude::*;
use std::f32::consts::PI;
use std::fmt::{Display, Formatter};

pub struct Missile {
    start_position: Vec2,
    pub position: Vec2,
    pub velocity: Vec2,
    angle: f32,
    pub is_alive: bool,
    pub lift_off_time: i32,
}

impl Missile {
    pub fn spawn() -> Self {
        // Füze için rastgele bir x noktası al.
        // Ekranın sağ ve sol taraflarından 4te 1 kırpılmış bir alan kullanılmakta.
        let x = rand::gen_range(
            screen_width() * 0.25,
            screen_width() - screen_width() * 0.25,
        );

        // Dikey vektör
        let c = Vec2::new(0., screen_height());
        // sol açı vektörü
        let a = Vec2::new(0. - x, screen_height());
        // sağ açı vektörü
        let b = Vec2::new(screen_width() - x, screen_height());
        // Nokta Çarpım ve vektör büyüklüklerine göre a ile c ve b ile c arasındaki açıları bul
        let mut left_angle = (a.dot(c) / (a.length() * c.length())).acos();
        let mut right_angle = (b.dot(c) / (b.length() * c.length())).acos();
        // Sol açıyı 90 derecenin üstü olacak şekilde ayarla
        left_angle += PI / 2.;
        // Sağ açıyı doksan dereceden düşük olacak şekilde ayarla
        right_angle = PI / 2. - right_angle;
        // println!(
        //     "Max LA {} Max RA {}",
        //     left_angle.to_degrees(),
        //     right_angle.to_degrees()
        // );

        // Bulunan sol ve sağ açı aralığına göre rastgele bir füze açısı al (Radyan cinsinden)
        let angle: f32 = rand::gen_range(right_angle, left_angle);

        Self {
            start_position: Vec2::new(x, 0.),
            position: Vec2::new(x, 0.),
            velocity: Vec2::new(angle.cos(), angle.sin()),
            angle,
            is_alive: true,
            lift_off_time: rand::gen_range(get_fps(), get_fps() + MAX_LIFT_OFF_TIME),
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
            RED,
        );
    }
}

impl Display for Missile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pos: {}, Dir: {}, Angle:{} rad,{} deg, lift off {}",
            self.position,
            self.velocity,
            self.angle,
            self.angle.to_degrees(),
            self.lift_off_time
        )
    }
}

pub fn create_missiles(quantity: i32) -> Vec<Missile> {
    let mut missiles = Vec::new();
    for _ in 0..quantity {
        let missile = Missile::spawn();
        //println!("{}", &missile);
        missiles.push(missile);
    }
    missiles
}
