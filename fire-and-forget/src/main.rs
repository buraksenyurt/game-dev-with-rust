use crate::rusty::Rusty;
use macroquad::color::Color;
use macroquad::input::{is_mouse_button_down, mouse_position, MouseButton};
use macroquad::prelude::{draw_texture, load_texture, rand, vec2};
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};

mod rusty;

#[macroquad::main("Fire and Forget")]
async fn main() {
    let ferris_texture = load_texture("assets/ferris.png").await.unwrap();
    let fire_point = vec2(
        screen_width() * 0.5 - ferris_texture.width() * 0.5,
        screen_height() - ferris_texture.height(),
    );
    let mut ferris: Rusty;
    loop {
        clear_background(Color::default());

        if is_mouse_button_down(MouseButton::Left) {
            ferris = Rusty {
                position: fire_point,
                speed: Default::default(),
                color: Color::from_rgba(
                    rand::gen_range(50, 240),
                    rand::gen_range(80, 240),
                    rand::gen_range(100, 240),
                    255,
                ),
            };
            let _m_pos = mouse_position();

            draw_texture(
                ferris_texture,
                ferris.position.x,
                ferris.position.y,
                ferris.color,
            );
        }

        next_frame().await
    }
}
