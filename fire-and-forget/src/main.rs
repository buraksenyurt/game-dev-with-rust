use crate::rusty::Rusty;
use macroquad::color::Color;
use macroquad::input::{is_mouse_button_down, mouse_position, MouseButton};
use macroquad::prelude::{draw_texture, load_texture, rand, vec2};
use macroquad::time::get_time;
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};
use std::f32::consts::PI;

mod rusty;

#[macroquad::main("Fire and Forget")]
async fn main() {
    let ferris_texture = load_texture("assets/ferris.png").await.unwrap();
    let ref_point = vec2(
        screen_width() * 0.5 - ferris_texture.width() * 0.5,
        screen_height() - (ferris_texture.height() + 10.),
    );
    let mut ferris = Rusty {
        start_position: ref_point,
        current_position: ref_point,
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

        ferris.current_position.x += 1. * angle.sin();
        ferris.current_position.y += 1. * -angle.cos();

        if ferris.current_position.x < 0.
            || ferris.current_position.x > screen_width()
            || ferris.current_position.y < 0.
        {
            ferris = Rusty {
                start_position: ref_point,
                current_position: ref_point,
                color: Default::default(),
            };
            angle = 0.;
            clickable = true;
        }

        draw_texture(
            ferris_texture,
            ferris.current_position.x,
            ferris.current_position.y,
            ferris.color,
        );
        next_frame().await
    }
}
