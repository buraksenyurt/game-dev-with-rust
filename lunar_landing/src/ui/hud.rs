use crate::constants::WIDTH;
use crate::game::Game;
use crate::ui::draw_text;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::{Duration, Instant};

pub struct Hud {
    fuel_warn_last_blink: Instant,
    fuel_warn_blink_interval: Duration,
    fuel_warn_blink_visible: bool,
    alt_warn_last_blink: Instant,
    alt_warn_blink_interval: Duration,
    alt_warn_blink_visible: bool,
}

impl Hud {
    pub fn new() -> Self {
        Self {
            fuel_warn_last_blink: Instant::now(),
            fuel_warn_blink_visible: true,
            fuel_warn_blink_interval: Duration::from_millis(750),
            alt_warn_last_blink: Instant::now(),
            alt_warn_blink_visible: true,
            alt_warn_blink_interval: Duration::from_millis(500),
        }
    }
    pub fn draw(&mut self, game: &Game, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let v_point = game.shuttle.velocity.to_point();
        draw_text(
            canvas,
            &format!("Fuel: {}", game.shuttle.fuel_level),
            14,
            Color::WHITE,
            WIDTH - 100,
            10,
        )?;
        draw_text(
            canvas,
            &format!(
                "Pos {}:{}",
                game.shuttle.position.x + v_point.x,
                game.shuttle.position.y + v_point.y
            ),
            14,
            Color::WHITE,
            WIDTH - 100,
            30,
        )?;
        for (i, dist) in game.distances.iter().enumerate() {
            draw_text(
                canvas,
                &format!("Dist{}:{:.2}", i + 1, dist),
                14,
                Color::GREEN,
                WIDTH - 100,
                60 + (i as i32 * 20),
            )?;
        }
        self.show_fuel_warning(&game, canvas)?;
        self.show_low_altitude_warning(&game, canvas)?;

        Ok(())
    }

    fn show_fuel_warning(
        &mut self,
        game: &Game,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), String> {
        if game.shuttle.is_fuel_critical() {
            let now = Instant::now();
            if now.duration_since(self.fuel_warn_last_blink) >= self.fuel_warn_blink_interval {
                self.fuel_warn_blink_visible = !self.fuel_warn_blink_visible;
                self.fuel_warn_last_blink = now;
            }
            if self.fuel_warn_blink_visible {
                draw_text(canvas, "Fuel Critical", 14, Color::RED, 30, 30)?;
            }
        }
        Ok(())
    }

    fn show_low_altitude_warning(
        &mut self,
        game: &Game,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), String> {
        if game.shuttle.is_low_altitude() {
            let now = Instant::now();
            if now.duration_since(self.alt_warn_last_blink) >= self.alt_warn_blink_interval {
                self.alt_warn_blink_visible = !self.alt_warn_blink_visible;
                self.alt_warn_last_blink = now;
            }
            if self.alt_warn_blink_visible {
                draw_text(canvas, "Altitude", 14, Color::RED, 30, 10)?;
            }
        }
        Ok(())
    }
}
