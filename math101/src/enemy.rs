use crate::constant::*;
use macroquad::prelude::*;

pub struct Enemy {
    pub rect: Rect,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5 - ENEMY_BOX_SIZE.x * 0.5,
                50.,
                ENEMY_BOX_SIZE.x,
                ENEMY_BOX_SIZE.y,
            ),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, MAGENTA);
    }
}
