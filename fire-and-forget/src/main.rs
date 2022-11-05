use crate::rusty::Rusty;
use macroquad::color::Color;
use macroquad::input::{is_mouse_button_down, mouse_position, MouseButton};
use macroquad::prelude::{draw_texture, load_texture, rand, vec2};
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};

mod rusty;

#[macroquad::main("Fire and Forget")]
async fn main() {
    let ferris_texture = load_texture("assets/ferris.png").await.unwrap();
    let ref_point = vec2(
        screen_width() * 0.5 - ferris_texture.width() * 0.5,
        screen_height() - (ferris_texture.height() + 10.),
    );
    let mut ferris = Rusty {
        position: ref_point,
        color: Default::default(),
    };
    let mut clickable = true;

    clear_background(Color::default());
    let mut angle = 0.;
    loop {
        if is_mouse_button_down(MouseButton::Left) {
            ferris.color = Color::from_rgba(
                rand::gen_range(50, 240),
                rand::gen_range(80, 240),
                rand::gen_range(100, 240),
                255,
            );
            if clickable {
                let m_pos = vec2(mouse_position().0, mouse_position().1);
                let opposite = m_pos.x - ref_point.x;
                let adjacent = (ref_point.y - m_pos.y).abs();
                angle = (opposite / adjacent).atan();
                //angle = m_pos.angle_between(ref_point) + PI / 2.;
                println!("{},{}", angle.to_degrees(), angle.to_radians());
                clickable = false;
            }
        }

        ferris.position.x += 1. * angle.sin();
        ferris.position.y += 1. * -angle.cos();

        if ferris.position.x < 0. || ferris.position.x > screen_width() || ferris.position.y < 0. {
            ferris = Rusty {
                position: ref_point,
                color: Default::default(),
            };
            clickable = true;
        }

        draw_texture(
            ferris_texture,
            ferris.position.x,
            ferris.position.y,
            ferris.color,
        );
        next_frame().await
    }
}
