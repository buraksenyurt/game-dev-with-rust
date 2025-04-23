use musi_lili::{Color, DISPLAY_HEIGHT, DISPLAY_WIDTH, Game, Key, Lili, Palette};

fn main() {
    musi_lili::run::<Commando>(musi_lili::load!("Commando Game"));
}

struct Commando;

impl Game for Commando {
    fn init(_lili: &Lili) -> Self {
        //lili.set_pal(Palette::Red);
        Self
    }
    fn update(&mut self, lili: &Lili) {}
    fn draw(&mut self, lili: &Lili) {
        lili.clear(0);
        lili.key(&Key::Space).then(|| {
            lili.rect(1, 1, 10, 10, 2);
        });
        lili.spr(10, 49, 38, false, false);
        lili.fill_circ(10, DISPLAY_HEIGHT as i32 - 3, 2, 3);
        lili.txt("Commando", DISPLAY_WIDTH as i32 / 2, 0, 2);
        lili.circ(DISPLAY_WIDTH as i32 / 2, DISPLAY_HEIGHT as i32 / 2, 10, 1);
        lili.line(DISPLAY_WIDTH as i32 / 2, DISPLAY_HEIGHT as i32- 10, 100, 80, 3);
    }
}
