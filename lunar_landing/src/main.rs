mod constants;
mod draw;
mod entity;
mod game;
mod math;
mod utility;

use crate::constants::*;
use crate::draw::*;
use crate::entity::Shuttle;
use crate::game::Game;
use crate::math::Vector;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut velocity = Vector::default();
    let mut shuttle = Shuttle::new();

    let window = video_subsystem
        .window("Lunar Landing 2049", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let game = Game::new();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    velocity.x -= 0.50;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    velocity.x += 0.50;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    velocity.y += 1.50;
                    shuttle.fuel_level -= 2;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    velocity.y -= 1.;
                    shuttle.fuel_level -= 10;
                }
                _ => {}
            }
        }

        //draw_landing_area(&mut canvas)?;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        draw_game_area(&mut canvas, &game)?;
        draw_landing_platforms(&mut canvas, &game)?;
        //draw_mountain(&mut canvas)?;
        let v_point = velocity.to_point();
        shuttle.draw(&mut canvas, Color::RGB(255, 255, 0), v_point)?;
        velocity.y += 0.05;
        shuttle.fuel_level -= 1;

        draw_text(
            &mut canvas,
            &ttf_context,
            &format!("Fuel: {}", shuttle.fuel_level),
            14,
            Color::RGBA(255, 255, 255, 255),
            WIDTH - 100,
            10,
        )?;
        draw_text(
            &mut canvas,
            &ttf_context,
            &format!(
                "({}:{})",
                shuttle.position.x + v_point.x,
                shuttle.position.y + v_point.y
            ),
            14,
            Color::RGBA(255, 255, 255, 255),
            WIDTH - 100,
            30,
        )?;

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
