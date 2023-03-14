use crate::common::contants::{EAT_PELLET_SCORE, EAT_POWERUP_SCORE, TILE_MAP_WIDTH};
use crate::common::direction::Direction;
use crate::common::position::Position;
use crate::entities::cell::{Tile, TileType};
use crate::entities::map::Map;
use crate::entities::score::Score;
use piston::Key::P;

pub struct Pacman {
    map: Map,
    pos: Position,
    dir: Direction,
    target_dir: Direction,
    ticks: u32,
    score: Score,
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
            score: Score::default(),
        }
    }
}

impl Pacman {
    // Pacman'in gideceği yönü ayarlayan fonksiyon
    pub fn set_direction(&mut self, candidate: Direction) {
        // Pacman'in geçerli canı var mı?
        if self.score.lives == 0 {
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
        if self.score.lives == 0 {
            return;
        }
        self.go();
        //TODO Hayaletler hareket ettirilecek
        //TODO Hayalet ile pacman arasındaki etkileşimler(çarpışma gibi) kontrol edilecek
    }

    pub fn get_location(&self) -> (Position, Direction) {
        (Position::new(self.pos.x, self.pos.y), self.target_dir)
    }

    // Pacman karakterini hareket ettiren fonksiyon
    fn go(&mut self) {
        // Hedeflenen yöne doğru gidebilir mi bakılır
        if self.can_turn() {
            self.dir = self.target_dir;
        }
        // Basılan tuşun işaret ettiği yöne göre yeni pozisyon bulunur
        let position = match self.dir {
            Direction::Up => Position::new(self.pos.x, self.pos.y - 1),
            Direction::Down => Position::new(self.pos.x, self.pos.y + 1),
            Direction::Left => Position::new(self.pos.x - 1, self.pos.y),
            Direction::Right => Position::new(self.pos.x + 1, self.pos.y),
        };
        // Elde edilen pozisyona göre haritada ne olduğuna bakılır
        match self.map.get(&position) {
            /*
               map.get'de bir pozisyon olmamasının tek şartı Pacman'in oyun sahası dışında olmasıdır.
               Pacman oyununda hatırlanacağı üzere tam ortada ekranın solundan veya sağından
               çıkılıp ters noktadan tekrar oyuna girilmesi mümkündür.

               Sol tarafan çıkılması halinde pacman'in x değeri -1 dir.
               Sağ taraftan çıkılma halinde ise bu değer sahanın genişliği ile eş değerdir.

            */
            None => {
                if position.x == -1 {
                    // Pacman sahanın solundan çıktıysa sağ uçtan tekrar giriş yapsın
                    self.pos.x = TILE_MAP_WIDTH as i32 - 1;
                } else if position.x == TILE_MAP_WIDTH as i32 {
                    // pacman sağ uçtan çıktıysa sol uçtan tekrar giriş yapsın
                    self.pos.x = 0;
                }
            }
            Some(Tile::NotWall(powerup)) => {
                // Duvar olmadığı sürece Pacman hareketine kesintisiz olarak devam eder.
                self.pos = position;
                // Bir PowerUp ile karşılaşıldığında TileType'a göre hareket edilir.
                match powerup {
                    // Nokta ile ifade edilen Pellet'ler Pacman'in yemeğe çalıştığı yemişlerdir
                    TileType::Dot => {
                        self.map.eat_something(position); // Hücreyi sahadan çıkarır
                        self.score.total_point += EAT_PELLET_SCORE;
                    }
                    TileType::Empty => {}
                    // Bir PowerUp söz konusu ise puan buna göre artırlır.
                    TileType::Powerup => {
                        self.map.eat_something(position); // Hücreyi sahadan çıkarır
                        self.score.total_point += EAT_POWERUP_SCORE;
                    }
                }
            }
            _ => (),
        }
    }
}
