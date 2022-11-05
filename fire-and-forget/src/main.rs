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
        screen_height() - ferris_texture.height(),
    );
    let mut ferris = Rusty {
        start_position: ref_point,
        current_position: ref_point,
        speed: Default::default(),
        direction: Default::default(),
        color: Default::default(),
    };
    clear_background(Color::default());
    let mut grade = 0.;
    let mut n: f32 = 0.;
    loop {
        if is_mouse_button_down(MouseButton::Left) {
            ferris.color = Color::from_rgba(
                rand::gen_range(50, 240),
                rand::gen_range(80, 240),
                rand::gen_range(100, 240),
                255,
            );
            let m_pos = vec2(mouse_position().0, mouse_position().1);
            // Mouse'a tıklanan yer ile ateşe başlangıç noktası arasındaki doğru denkleminin bulunması
            // Eğimi bul
            grade = (ref_point.y - m_pos.y).abs() / (ref_point.x - m_pos.x).abs();
            if m_pos.y < ref_point.y {
                if m_pos.x > ref_point.x {
                    grade = -grade;
                } else {
                    grade = grade;
                }
            } else if m_pos.y == ref_point.y {
                grade = 0.;
            }
            n = ref_point.y - (grade * ref_point.x);
            println!(
                "Grade is {}\nFerris {}\nMouse {}",
                grade, ferris.current_position, m_pos
            );
            // y -  referans noktanın y değeri = grade x (x - referans noktanın x değeri)
        }
        if grade < 0. {
            ferris.current_position.x += 1.;
        } else {
            ferris.current_position.x -= 1.;
        }
        ferris.current_position.y = grade * ferris.current_position.x + n;

        if ferris.current_position.x < 0.
            || ferris.current_position.x > screen_width()
            || ferris.current_position.y < 0.
        {
            ferris = Rusty {
                start_position: ref_point,
                current_position: ref_point,
                speed: Default::default(),
                direction: Default::default(),
                color: Default::default(),
            };
        }
        // println!("Current Position x,y {}", ferris.current_position);
        // println!("Start Position x,y {}", ferris.current_position);

        draw_texture(
            ferris_texture,
            ferris.current_position.x,
            ferris.current_position.y,
            ferris.color,
        );

        next_frame().await
    }
}
