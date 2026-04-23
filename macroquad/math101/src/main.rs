extern crate core;

mod constant;
mod enemy;
mod pisagor;
mod plane;
mod player;
mod robot;

use crate::enemy::Enemy;
use crate::pisagor::calculate_distance;
use crate::plane::Plane;
use crate::player::Player;
use crate::robot::Robot;
use macroquad::prelude::*;

#[macroquad::main("Math 101")]
async fn main() {
    let mut player = Player::new();
    let enemy = Enemy::new();
    let mut robot = Robot::new(0., screen_height() * 0.5);
    let mut plane = Plane::new(screen_width() * 0.25, screen_height() * 0.5, 100.);

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
        robot.update();
        robot.draw();
        plane.update();
        plane.draw();

        next_frame().await
    }
}
