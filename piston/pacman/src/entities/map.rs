/*
   Oyun sahası sabit ve çizimi için, hangi hücrede ne olduğunu bilmek için
   aşağıdaki ASCII haritası kullanılmakta.

   #'ler duvarları,
   .'lar hareket edilebilen yerleri(ki buralarda elma, armut veya başka bonus'lar olabilir)
   R ile işaretlenmiş kısımlar oyun sahasının ortasındaki oda zemini göstermek için kullanılıyor.
   P harfleri powerup'ları işaret etmekte.
*/

use crate::common::contants::{TILE_MAP_HEIGHT, TILE_MAP_WIDTH};
use crate::common::position::Position;
use crate::entities::cell::{Tile, TileType};
use crate::entities::line::Line;
use crate::entities::pellets::Pellets;

const TILES_MAP: [&str; 31] = [
    "############################",
    "#............##............#",
    "#.####.#####.##.#####.####.#",
    "#P####.#####.##.#####.####P#",
    "#.####.#####.##.#####.####.#",
    "#..........................#",
    "#.####.##.########.##.####.#",
    "#.####.##.########.##.####.#",
    "#......##....##....##......#",
    "######.##### ## #####.######",
    "######.##### ## #####.######",
    "######.##          ##.######",
    "######.## ###RR### ##.######",
    "######.## #RRRRRR# ##.######",
    "      .   #RRRRRR#   .      ",
    "######.## #RRRRRR# ##.######",
    "######.## ######## ##.######",
    "######.##          ##.######",
    "######.## ######## ##.######",
    "######.## ######## ##.######",
    "#............##............#",
    "#.####.#####.##.#####.####.#",
    "#.####.#####.##.#####.####.#",
    "#P..##................##..P#",
    "###.##.##.########.##.##.###",
    "###.##.##.########.##.##.###",
    "#......##....##....##......#",
    "#.##########.##.##########.#",
    "#.##########.##.##########.#",
    "#..........................#",
    "############################",
];

/*
    Oyun sahasını temsil eden veri yapısı
    Matris boyutlarında her bir zemini içeren bir dizi kullanıyor
    Pacman'in yediği küçük parçalar pellet olarak adlandırılmakta
    Oyun sahasında kaç tane pellet olduğu ve konumları da bu veri yapısı ile taşınıyor.
    Pellet'ten kasıt Pacman'in geçtiği yerlerde yer alan minik noktalar.
    Onları yedikçe puan kazanır.
*/
pub struct Map {
    pub tiles: Vec<Tile>,
    pub pellets: Pellets,
}

// Sahayı hazırlayan default trait uyarlaması
impl Default for Map {
    fn default() -> Self {
        /*
           İlk önce string haritayı okuyup TileSet'i hazırlamak lazım.
           TILES_MAP satır satır inen metinsel bir içerik.
           flat_map ile her bir satırdaki karakterleri dolaşmayı sağlayan bir iterator yakalanıyor.
           filter_map ile de bu satırdaki herbir karakteri dolaşan iterator.
           pattern matching ile c'nin ne olduğuna bakıp bir Tile değeri hazırlanıyor.
           collect ile de bunlar tile_set vektöründe toplanıyorlar.
        */
        let tile_set: Vec<Tile> = TILES_MAP
            .iter()
            .flat_map(|line| line.chars())
            .filter_map(|c| match c {
                '#' => Some(Tile::Wall),
                '.' => Some(Tile::NotWall(TileType::Dot)),
                ' ' => Some(Tile::NotWall(TileType::Empty)),
                'P' => Some(Tile::NotWall(TileType::Powerup)),
                'R' => Some(Tile::Room),
                _ => None,
            })
            .collect();
        /*
           Yemişlerin toplam sayısını buluyoruz. Tile_Set elemanlarını gezerken
           o elemanın Dot olup olmadığını kontrol etmek yeterli
        */
        let pellets_count = tile_set
            .iter()
            .filter(|c| Tile::NotWall(TileType::Dot) == **c)
            .count() as u32;

        Self {
            tiles: tile_set,
            pellets: Pellets::new(pellets_count, get_pellet_coordinates()),
        }
    }
}

/*
   Yemişlerin(noktalar) olduğu yerlerin koordinatlarını bulmak için kullanılan yardımcı fonksiyon.
   TILES_MAP'in tüm satırları ve satırlardaki karakterleri(yani sütunları) tek tek dolaşılıyor.
   . ile karşılaşıldığında elde edilen x,y değerleri yeni bir vektörde toplanıyor.

   fold fonksiyonu ile collect sırasında toplanan v
*/
fn get_pellet_coordinates() -> Vec<(usize, usize)> {
    TILES_MAP
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '.')
                .map(move |(x, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .fold(vec![], |mut acc, mut l| {
            acc.append(&mut l);
            acc
        })
}

impl Map {
    // Map matrisini satır satır dolaşmamızı sağlayan fonksiyon
    pub fn read_lines(&self) -> Line {
        Line {
            map: self,
            index: 0,
        }
    }

    // Matriste parametre olarak gelen konumdaki değeri döndüren fonksiyon
    // Eğer koordinatlar 0 dan küçük veya matris ebatlarından büyükse None döner.
    // Aksi halde x ve y hesaplamaya katılarak vektördeki indisin olduğu Tile nesnesi döndürülür.
    pub fn get(&self, pos: &Position) -> Option<Tile> {
        if (pos.x < 0 || pos.x >= TILE_MAP_WIDTH as i32)
            || (pos.y < 0 || pos.y >= TILE_MAP_HEIGHT as i32)
        {
            None
        } else {
            Some(self.tiles[TILE_MAP_WIDTH * pos.y as usize + pos.x as usize])
        }
    }

    // Pacman bir noktadan geçerken pellet veya farklı bir powerup
    // olup olmadığının kontrolünün yaptığımız fonksiyon.
    pub fn eat_something(&mut self, pos: Position) {
        // Eğer bir nokta ile karşılaşmışsa bu yenmeli ve toplam nokta(pellet) sayısı azaltılmalı
        if let Some(Tile::NotWall(TileType::Dot)) = self.get(&pos) {
            self.pellets.count -= 1;
        }
        // Sonrasında geçilen kare boş bir zemin ile değiştirilir
        self.tiles[TILE_MAP_WIDTH * pos.y as usize + pos.x as usize] =
            Tile::NotWall(TileType::Empty);
    }
}
