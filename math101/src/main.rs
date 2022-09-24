extern crate core;

mod constant;
mod player;

use crate::player::Player;
use macroquad::prelude::*;

#[macroquad::main("Math 101")]
async fn main() {
    let mut player = Player::new();

    loop {
        clear_background(BLACK);
        player.update(get_frame_time());
        player.draw();
        next_frame().await
    }
}
