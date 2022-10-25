mod resource;
mod tank;

use crate::resource::TANK_TEXTURE;
use crate::tank::Tank;
use macroquad::prelude::*;

#[macroquad::main("Wolf Tank")]
async fn main() {
    let tank_texture: Texture2D = load_texture(TANK_TEXTURE).await.unwrap();
    let mut player_tank = Tank::new(tank_texture);

    println!("{}", player_tank);
    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Right) {
            player_tank.rotation += 0.1;
            //println!("{}", player_tank);
        } else if is_key_down(KeyCode::Left) {
            player_tank.rotation -= 0.1;
            //println!("{}", player_tank);
        }
        player_tank.draw();

        next_frame().await
    }
}
