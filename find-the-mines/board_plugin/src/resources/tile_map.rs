use crate::resources::tile::Tile;
use rand::{thread_rng, Rng};
use std::ops::{Deref, DerefMut};
use crate::components::Coordinates;

/*
   Ekran iki boyutlu bir array gibi düşünülürse zemin haritasını tutmamız gerekecektir.
*/
pub struct TileMap {
    pub mine_count: u16,
    pub height: u16,
    pub width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    // Boş bir zemin seti oluşturmak için kullanılan fonksiyon
    pub fn init(width: u16, height: u16) -> Self {
        // Aşağıdaki teknik oldukça etkili. High Order Function'lar kullanılarak
        // döngüye bile başvurmadan bir vector oluşturuluyor
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Empty).collect())
            .collect();
        Self {
            mine_count: 0,
            height,
            width,
            map,
        }
    }

    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinates + tuple) // Coordinate veri yapısı için add traiti implement edilmişti.
    }

    // Gelen koordinatlarda bir mayın var mı kontrolünü yapan fonksiyon
    pub fn is_mine_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x >= self.width || coordinates.y >= self.height {
            return false;
        };
        self.map[coordinates.y as usize][coordinates.x as usize].is_mine()
    }

    pub fn mine_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_mine_at(coordinates) {
            return 0;
        }
        let count = self
            .safe_square_at(coordinates)
            .filter(|coord| self.is_mine_at(*coord))
            .count();
        count as u8
    }

    // Rastgele konumlara mayın yerleştiren fonksiyon
    pub fn setup_mines(&mut self, mine_count: u16) {
        self.mine_count = mine_count;
        let mut remaining_mines = mine_count;
        let mut rng = thread_rng();
        while remaining_mines > 0 {
            let (x, y) = (
                rng.gen_range(0..self.width) as usize,
                rng.gen_range(0..self.height) as usize,
            );
            if let Tile::Empty = self[y][x] {
                self[y][x] = Tile::Mine;
                remaining_mines -= 1;
            }
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let coords = Coordinates { x, y };
                if self.is_mine_at(coords) {
                    continue;
                }
                let num = self.mine_count_at(coords);
                if num == 0 {
                    continue;
                }
                let tile = &mut self[y as usize][x as usize];
                *tile = Tile::MineNeighbor(num);
            }
        }
    }
}

// Map vektörünü dereferans etmek için Deref trait'ini uyguladık
impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

/*
   Üzerinde bulunulan karenin çevresini tanımlayan bir tuple dizisi söz konusu.
   Aşağıdaki gibi bir sistem kullanılmakta.

   *--------*-------*-------*
   | -1, 1  | 0, 1  | 1, 1  |
   |--------|-------|-------|
   | -1, 0  | tile  | 1, 0  |
   |--------|-------|-------|
   | -1, -1 | 0, -1 | 1, -1 |
   *--------*-------*-------*
*/
const SQUARE_COORDINATES: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];
