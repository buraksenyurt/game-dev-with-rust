mod constants;
mod draw;
mod entity;
mod ext_factors;
mod game;
mod math;
mod utility;

use crate::constants::*;
use crate::draw::*;
use crate::entity::Shuttle;
use crate::ext_factors::ExternalFactors;
use crate::game::Game;
use crate::math::Vector;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let factors = ExternalFactors {};
    let mut shuttle = Shuttle::new();

    let window = video_subsystem
        .window("Lunar Landing 2049", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut game = Game::new();
    let mut last_update = Instant::now();

    'running: loop {
        let frame_duration = Duration::new(0, 1_000_000_000u32 / 60);
        let now = Instant::now();
        let delta = now.duration_since(last_update);
        last_update = now;
        let delta_seconds = delta.as_secs_f32();
        if frame_duration > delta {
            std::thread::sleep(frame_duration - delta);
        }

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
                    shuttle.velocity.x -= 30. * delta_seconds;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    shuttle.velocity.x += 30. * delta_seconds;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    shuttle.velocity.y += 75. * delta_seconds;
                    shuttle.fuel_level -= 2;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    shuttle.velocity.y -= 50. * delta_seconds;
                    shuttle.fuel_level -= 10;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        game.draw(&mut canvas)?;
        draw_landing_platforms(&mut canvas, &game)?;
        shuttle.draw(&mut canvas, Color::RGB(255, 255, 0))?;
        draw_hud(&shuttle, &mut canvas)?;
        if !shuttle.is_landed(&game) {
            factors.toss_randomly(&mut shuttle, Vector { x: 40., y: 80. }, delta_seconds);
            shuttle.velocity.y += 2.5 * delta_seconds;
            shuttle.fuel_level -= 1;
        }
        game.move_meteors(delta_seconds);
        draw_meteors(&mut canvas, &game)?;

        canvas.present();
    }

    Ok(())
}
