use crate::rusty::Rusty;
use macroquad::color::Color;
use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::prelude::{
    draw_line, draw_texture, draw_triangle, load_texture, rand, vec2, Vec2, DARKGREEN, WHITE,
};
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};
use std::f32::consts::PI;

mod rusty;

const LAUNCHER_RADIUS: f32 = 36.;
const BARREL_LENGTH: f32 = 58.;
const BARREL_THICKNESS: f32 = 6.;
const MISSILE_SPEED: f32 = 4.;
const LAUNCHER_BASE_COLOR: Color = Color::from_rgba(150, 50, 210, 255);

#[macroquad::main("Fire and Forget")]
async fn main() {
    let ferris_texture = load_texture(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/ferris.png"))
        .await
        .unwrap();
    let launcher_origin = vec2(screen_width() * 0.5, screen_height() - 20.);
    let mut ferris = Rusty {
        position: vec2(-500., -500.),
        color: WHITE,
    };
    let mut missile_active = false;
    let mut launch_direction = vec2(0., -1.);

    loop {
        clear_background(Color::default());

        let mouse = vec2(mouse_position().0, mouse_position().1);
        let aim_direction = find_aim_direction(mouse - launcher_origin);
        let muzzle = launcher_origin + aim_direction * BARREL_LENGTH;

        draw_launcher(launcher_origin, muzzle);

        if !missile_active && is_mouse_button_pressed(MouseButton::Left) {
            launch_direction = aim_direction;
            ferris.position = vec2(
                muzzle.x - ferris_texture.width() * 0.5,
                muzzle.y - ferris_texture.height() * 0.5,
            );
            ferris.color = Color::from_rgba(
                rand::gen_range(50, 240),
                rand::gen_range(80, 240),
                rand::gen_range(100, 240),
                255,
            );
            missile_active = true;
        }

        if missile_active {
            ferris.position += launch_direction * MISSILE_SPEED;
        } else {
            ferris.position = vec2(
                launcher_origin.x - ferris_texture.width() * 0.5,
                launcher_origin.y - ferris_texture.height() * 0.5,
            );
            ferris.color = WHITE;
        }

        if ferris.position.x + ferris_texture.width() < 0.
            || ferris.position.x > screen_width()
            || ferris.position.y + ferris_texture.height() < 0.
        {
            missile_active = false;
        }

        draw_texture(
            &ferris_texture,
            ferris.position.x,
            ferris.position.y,
            ferris.color,
        );
        next_frame().await
    }
}

fn draw_launcher(origin: Vec2, muzzle: Vec2) {
    draw_half_circle_filled(origin, LAUNCHER_RADIUS, 40, LAUNCHER_BASE_COLOR);
    draw_half_circle_outline(origin, LAUNCHER_RADIUS, 40, DARKGREEN);
    draw_line(
        origin.x,
        origin.y,
        muzzle.x,
        muzzle.y,
        BARREL_THICKNESS,
        DARKGREEN,
    );
}

fn draw_half_circle_filled(center: Vec2, radius: f32, segments: i32, color: Color) {
    for i in 0..segments {
        let t0 = PI + (i as f32 / segments as f32) * PI;
        let t1 = PI + ((i + 1) as f32 / segments as f32) * PI;
        let p0 = vec2(center.x + radius * t0.cos(), center.y + radius * t0.sin());
        let p1 = vec2(center.x + radius * t1.cos(), center.y + radius * t1.sin());
        draw_triangle(center, p0, p1, color);
    }
}

fn draw_half_circle_outline(center: Vec2, radius: f32, segments: i32, color: Color) {
    for i in 0..segments {
        let t0 = PI + (i as f32 / segments as f32) * PI;
        let t1 = PI + ((i + 1) as f32 / segments as f32) * PI;
        let p0 = vec2(center.x + radius * t0.cos(), center.y + radius * t0.sin());
        let p1 = vec2(center.x + radius * t1.cos(), center.y + radius * t1.sin());
        draw_line(p0.x, p0.y, p1.x, p1.y, 3., color);
    }
}

fn find_aim_direction(raw_direction: Vec2) -> Vec2 {
    if raw_direction.length_squared() < 0.0001 {
        return vec2(0., -1.);
    }

    let mut unit = raw_direction.normalize();
    // Namlunun aşağı bakmasını engelle
    if unit.y > -0.2 {
        unit.y = -0.2;
        unit = unit.normalize();
    }
    unit
}
