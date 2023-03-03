use crate::common::direction::Direction;
use crate::common::position::Position;
use crate::entities::cell::Tile;
use crate::entities::map::Map;

pub struct Pacman {
    map: Map,
    pos: Position,
    dir: Direction,
    target_dir: Direction,
    ticks: u32,
    score: u32,
    lives: u8,
}

impl Pacman {
    pub fn get_map(&self) -> &Map {
        &self.map
    }
}

impl Default for Pacman {
    fn default() -> Self {
        Self {
            map: Map::default(),
            pos: Position::default(),
            dir: Direction::Left,
            target_dir: Direction::Left,
            ticks: 0,
            lives: 3,
            score: 0,
        }
    }
}

impl Pacman {
    // Pacman'in gideceği yönü ayarlayan fonksiyon
    pub fn set_direction(&mut self, candidate: Direction) {
        // Pacman'in geçerli canı var mı?
        if self.lives == 0 {
            return;
        }
        // Hedef koordinatları yönlendir
        self.target_dir = candidate;
        // Hedef koordinatlar uygunsa ata
        if self.can_turn() {
            self.dir = self.target_dir;
        }
    }

    // Pacman tuşla basılan konuma gidebilir mi kontrolünü yapan fonksiyon.
    // target_dir = gidilmek istenen yönü ifade ediyor.
    fn can_turn(&self) -> bool {
        let (x, y) = match self.target_dir {
            Direction::Up => (self.pos.x, self.pos.y - 1),
            Direction::Down => (self.pos.x, self.pos.y + 1),
            Direction::Left => (self.pos.x - 1, self.pos.y),
            Direction::Right => (self.pos.x + 1, self.pos.y),
        };
        !matches!(self.map.get(&Position::new(x, y)), None | Some(Tile::Wall))
    }

    // oyun döngüsündeki kilit fonksiyon. Her tick event'i gerçekleştiğinde bazı aksiyonlar alınır.
    pub fn tick(&mut self) {
        self.ticks += 1;
        //TODO Tüm noktalar yenmiş mi kontrolü yapılacak

        // Pacman'in canları tükendiyse bu fonksiyon daha fazla çalışmasın
        if self.lives == 0 {
            return;
        }
        //TODO Pacman hareket ettirilecek
        //TODO Hayaletler hareket ettirilecek
        //TODO Hayalet ile pacman arasındaki etkileşimler(çarpışma gibi) kontrol edilecek
    }
}
