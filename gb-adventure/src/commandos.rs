use crate::common::Direction;
use crate::model::Player;
use musi_lili::{DISPLAY_WIDTH, Game, Key, Lili};

pub struct Commando {
    pub hero: Player,
    pub title: String,
}
impl Game for Commando {
    fn init(_lili: &Lili) -> Self {
        Self {
            hero: Player::new(),
            title: "Circle Commandos".to_string(),
        }
    }
    fn update(&mut self, lili: &Lili) {
        lili.key(&Key::RightArrow).then(|| {
            self.hero.update(Direction::Right);
        });
        lili.key(&Key::LeftArrow).then(|| {
            self.hero.update(Direction::Left);
        });
        lili.key(&Key::UpArrow).then(|| {
            self.hero.update(Direction::Up);
        });
        lili.key(&Key::DownArrow).then(|| {
            self.hero.update(Direction::Down);
        });
    }
    fn draw(&mut self, lili: &Lili) {
        lili.clear(0);
        lili.txt(
            "Circle Commandos",
            DISPLAY_WIDTH as i32 / 2 - self.title.len() as i32 * 2,
            0,
            2,
        );
        self.hero.draw(lili);
    }
}
