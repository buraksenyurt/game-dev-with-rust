use musi_lili::{DISPLAY_HEIGHT, DISPLAY_WIDTH, Game, Key, Lili};

fn main() {
    musi_lili::run::<Commando>(musi_lili::load!("Commando Game"));
}

struct Commando {
    hero: Hero,
    title: String,
}

struct Position {
    x: i32,
    y: i32,
}

struct Hero {
    position: Position,
    radius: u32,
    speed: i32,
}

impl Hero {
    pub fn new() -> Self {
        Hero {
            position: Position { x: 10, y: 10 },
            radius: 4,
            speed: 1,
        }
    }
    pub fn draw(&self, lili: &Lili) {
        lili.fill_circ(self.position.x, self.position.y, self.radius, 3);
    }
    pub fn update(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                if self.position.x > self.radius as i32 {
                    self.position.x -= self.speed;
                }
            }
            Direction::Right => {
                if self.position.x < DISPLAY_WIDTH as i32 - self.radius as i32 {
                    self.position.x += self.speed;
                }
            }
            Direction::Up => {
                if self.position.y > self.radius as i32 {
                    self.position.y -= self.speed;
                }
            }
            Direction::Down => {
                if self.position.y < DISPLAY_HEIGHT as i32 - self.radius as i32 {
                    self.position.y += self.speed;
                }
            }
        }
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Game for Commando {
    fn init(_lili: &Lili) -> Self {
        //lili.set_pal(Palette::Red);
        Self {
            hero: Hero::new(),
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
        // lili.key(&Key::Space).then(|| {
        //     lili.rect(1, 1, 10, 10, 2);
        // });
        //lili.key(&Key::RightArrow).then(||{
        self.hero.draw(lili);
        //});
        // lili.spr(10, 49, 38, false, false);
        // lili.fill_circ(10, DISPLAY_HEIGHT as i32 - 3, 2, 3);
        // lili.circ(DISPLAY_WIDTH as i32 / 2, DISPLAY_HEIGHT as i32 / 2, 10, 1);
        // lili.line(
        //     DISPLAY_WIDTH as i32 / 2,
        //     DISPLAY_HEIGHT as i32 - 10,
        //     100,
        //     80,
        //     3,
        // );
    }
}
