extern crate core;

mod constant;
mod enemy;
mod player;

use crate::enemy::Enemy;
use crate::player::Player;
use macroquad::prelude::*;

#[macroquad::main("Math 101")]
async fn main() {
    let mut player = Player::new();
    let enemy = Enemy::new();

    loop {
        clear_background(BLACK);

        // Pisagor ölçümü eklenecek

        player.update(get_frame_time());
        player.draw();
        enemy.draw();
        next_frame().await
    }
}
