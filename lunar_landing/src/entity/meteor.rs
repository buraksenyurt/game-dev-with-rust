use crate::constants::{HEIGHT, WIDTH};
use crate::entity::meteor_type::MeteorType;
use crate::entity::vector::Vector;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use std::f64::consts::PI;

#[derive(PartialEq)]
pub struct Meteor {
    pub center: Point,
    pub sides: u8,
    pub radius: i16,
    pub rot_angle: f64,
    pub velocity: Vector,
    pub in_range: bool,
    pub kind: MeteorType,
}

impl Meteor {
    pub fn new(center: Point, sides: u8, radius: i16, rot_angle: f64, in_range: bool) -> Self {
        let mut rng = thread_rng();
        let kinds = [
            MeteorType::RightBottomCorner,
            MeteorType::LeftBottomCorner,
            MeteorType::Vertical,
        ];
        let kind = *kinds.choose(&mut rng).unwrap();
        Self {
            center,
            sides,
            radius,
            rot_angle,
            velocity: Vector::default(),
            in_range,
            kind,
        }
    }
    pub fn mark_range(&mut self) {
        let curr_x = self.center.x + self.velocity.x as i32;
        let curr_y = self.center.y + self.velocity.y as i32;
        //println!("{curr_x}:{curr_y}");
        if curr_x > WIDTH || curr_y > HEIGHT {
            self.in_range = false;
            //println!("Meteor is out of range");
        }
        if curr_x < 0 {
            self.in_range = false;
            //println!("Meteor is out of range");
        }
    }
    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let mut points = Vec::new();
        let angle_step = 2.0 * PI / self.sides as f64;
        let angle_radian = self.rot_angle * PI / 180.;
        let velocity = self.velocity.to_point();
        for i in 0..self.sides {
            let angle = angle_step * i as f64 + angle_radian;
            let x = (self.radius as f64 * angle.cos()).round() as i32 + self.center.x + velocity.x;
            let y = (self.radius as f64 * angle.sin()).round() as i32 + self.center.y + velocity.y;
            points.push((x, y));
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for i in 0..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = if i == points.len() - 1 {
                points[0]
            } else {
                points[i + 1]
            };

            canvas.draw_line(Point::new(x1, y1), Point::new(x2, y2))?;
        }

        Ok(())
    }
}
