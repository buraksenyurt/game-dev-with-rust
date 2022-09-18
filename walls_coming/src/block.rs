use crate::constant::BLOCK_SIZE;
use macroquad::prelude::*;

pub struct Block {
    pub rect: Rect,
    // Bloğun sahip olduğu güce göre ekrandan kaldırılması sağlanabilir.
    // Top bir bloğa çarptığında 1 azaltılır örneğin. 0 gücü olanlar sahneden kaldırılır.
    pub strength: i32,
}

impl Block {
    pub fn new(position: Vec2) -> Self {
        Self {
            rect: Rect::new(position.x, position.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            strength: 1,
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}
