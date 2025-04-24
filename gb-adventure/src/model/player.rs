use crate::common::*;
use musi_lili::{DISPLAY_HEIGHT, DISPLAY_WIDTH, Lili};

pub struct Player {
    pub position: Position,
    pub radius: u32,
    pub speed: i32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Position { x: 10, y: 10 },
            radius: 4,
            speed: 1,
        }
    }
    pub fn draw(&self, lili: &Lili) {
        lili.fill_circ(self.position.x, self.position.y, self.radius, 3);
    }
    pub fn update(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                if self.position.x > self.radius as i32 {
                    self.position.x -= self.speed;
                }
            }
            Direction::Right => {
                if self.position.x < DISPLAY_WIDTH as i32 - self.radius as i32 {
                    self.position.x += self.speed;
                }
            }
            Direction::Up => {
                if self.position.y > self.radius as i32 {
                    self.position.y -= self.speed;
                }
            }
            Direction::Down => {
                if self.position.y < DISPLAY_HEIGHT as i32 - self.radius as i32 {
                    self.position.y += self.speed;
                }
            }
        }
    }
}
