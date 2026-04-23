use crate::constant::BLOCK_SIZE;
use macroquad::prelude::*;

pub enum BlockType {
    Brick,
    Stone,
    Iron,
    Bonus(Powerup),
}
pub enum Powerup {
    YaoMing,
    SpudWebb,
    CaptainSlow,
}

pub struct Block {
    pub rect: Rect,
    // Bloğun sahip olduğu güce göre ekrandan kaldırılması sağlanabilir.
    // Top bir bloğa çarptığında 1 azaltılır örneğin. 0 gücü olanlar sahneden kaldırılır.
    pub strength: i32,
    pub block_type: BlockType,
}

impl Block {
    pub fn new(position: Vec2) -> Self {
        let s: i32 = rand::gen_range(1, 4);

        let bt: BlockType = match s {
            1 => BlockType::Brick,
            2 => BlockType::Stone,
            3 => BlockType::Iron,
            _ => BlockType::Brick,
        };
        Self {
            rect: Rect::new(position.x, position.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            strength: s,
            block_type: bt,
        }
    }
    pub fn draw(&self) {
        let color: Color = match &self.block_type {
            BlockType::Brick => Color::from_rgba(220, 85, 57, 250),
            BlockType::Stone => Color::from_rgba(183, 176, 156, 250),
            BlockType::Iron => Color::from_rgba(161, 157, 148, 250),
            BlockType::Bonus(p) => match p {
                Powerup::YaoMing => Color::from_rgba(251, 191, 0, 250),
                Powerup::SpudWebb => Color::from_rgba(0, 206, 209, 150),
                Powerup::CaptainSlow => Color::from_rgba(92, 123, 209, 250),
            },
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}
