use crate::factory::{GameObject, MainState};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[derive(Default)]
pub struct Game;

impl GameObject for Game {
    fn update(
        &mut self,
        event_pump: &mut EventPump,
        _canvas: &mut Canvas<Window>,
        _delta_time: Duration,
    ) -> MainState {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return MainState::Exit;
                }
                _ => {}
            }
        }
        MainState::Running
    }
}
