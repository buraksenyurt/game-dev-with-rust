/*
   Oyunu yöneten veri yapısı olarak düşünebilir.
   Klavye olaylarının yönetilmesi, yeni oyun başlatılması, haritanın alınması,
   istatistiklerin toplanması hayalet ve pacman nesnelerinin konumlarının bulunması vs
   olayları yönettiğimiz nesne.
*/
use crate::common::direction::Direction;
use crate::common::position::Position;
use crate::entities::ghosts::Ghost;
use crate::entities::map::Map;
use crate::entities::pacman::Pacman;
use piston::Button::Keyboard;
use piston::{Event, Key, PressEvent, UpdateEvent};

pub struct Controller {
    game: Pacman,
    delta_time: f64,
}

impl Controller {
    pub fn new(game: Pacman) -> Self {
        Self {
            game,
            delta_time: 0.,
        }
    }

    // Klavye hareketlerini kontrol ettiğimiz fonksiyon
    pub fn event(&mut self, event: &Event) -> bool {
        if let Some(key) = event.press_args() {
            match key {
                Keyboard(Key::Up) => self.game.set_direction(Direction::Up),
                Keyboard(Key::Left) => self.game.set_direction(Direction::Left),
                Keyboard(Key::Right) => self.game.set_direction(Direction::Right),
                Keyboard(Key::Down) => self.game.set_direction(Direction::Down),
                _ => (),
            }
        }

        if let Some(update) = event.update_args() {
            self.delta_time += update.dt;
            if self.delta_time > 0.25 {
                self.delta_time -= 0.25;
                self.game.tick();
            }
        }

        false
    }

    pub fn get_map(&self) -> &Map {
        self.game.get_map()
    }

    pub fn get_pacman(&self) -> (Position, Direction) {
        self.game.get_location()
    }

    pub fn get_ghosts(&self) -> &[Ghost] {
        self.game.get_ghosts()
    }
}
