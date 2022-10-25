use macroquad::prelude::*;

#[macroquad::main("Wolf Tank")]
async fn main() {
    let texture: Texture2D = load_texture("./assets/tank.png").await.unwrap();

    loop {
        clear_background(BLACK);
        draw_texture(
            texture,
            screen_width() * 0.5 - texture.width() * 0.5,
            screen_height() * 0.5 - texture.height() * 0.5,
            WHITE,
        );
        next_frame().await
    }
}
