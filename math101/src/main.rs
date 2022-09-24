extern crate core;

mod constant;
mod enemy;
mod pisagor;
mod player;

use crate::enemy::Enemy;
use crate::pisagor::calculate_distance;
use crate::player::Player;
use macroquad::prelude::*;

#[macroquad::main("Math 101")]
async fn main() {
    let mut player = Player::new();
    let enemy = Enemy::new();

    loop {
        clear_background(BLACK);

        let d = calculate_distance(
            player.circle.x,
            player.circle.y,
            enemy.rect.x + enemy.rect.w * 0.5,
            enemy.rect.y + enemy.rect.h * 0.5,
        );
        if d < 50. {
            let info = format!("Mesafe {}", d);
            let scale = measure_text(&info, None, 48, 1.);
            draw_text_ex(
                &info,
                0.,
                screen_height() * 0.5 - scale.height * 0.5,
                TextParams {
                    font_size: 24,
                    color: GOLD,
                    ..Default::default()
                },
            );
        }

        player.update(get_frame_time());
        player.draw();
        enemy.draw();
        next_frame().await
    }
}
