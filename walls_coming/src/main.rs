mod constant;

use crate::constant::PLAYER_BOX_SIZE;
use macroquad::prelude::*;

#[macroquad::main("WallsComing")]
async fn main() {
    loop {
        let player_rect = Rect::new(
            screen_width() * 0.5 - PLAYER_BOX_SIZE.x * 0.5,
            screen_height() - 30.,
            PLAYER_BOX_SIZE.x,
            PLAYER_BOX_SIZE.y,
        );

        clear_background(WHITE);
        draw_rectangle(
            player_rect.x,
            player_rect.y,
            player_rect.w,
            player_rect.h,
            SKYBLUE,
        );
        next_frame().await
    }
}
