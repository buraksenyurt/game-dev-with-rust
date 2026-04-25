use crate::common::contants::TILE_MAP_WIDTH;
use crate::common::direction::Direction;
use crate::common::position::Position;
use crate::entities::cell::Tile;
use crate::entities::map::Map;

pub enum GhostMode {
    Pursue,  // Pacman'i takip eden kabiliyetteki hayalet türünü temsil eder
    Scatter, // belli bir mıntıkada dolanan hayaletleri temsil eder
}

// Oyunda 4 tür hayalet var. Asset'ler de aynı isimdeler.
pub enum GhostType {
    Blinky,
    Clyde,
    Inky,
    Pinky,
}

// Oyundaki bir hayaleti temsil etmek için kullanılan veri yapısı
pub struct Ghost {
    kind: GhostType,
    pos: Position,
    last_pos: Position,
    house_timer: u16,
    dir: Direction,
    rng_state: u32,
}

impl Ghost {
    pub fn new(kind: GhostType) -> Self {
        let start_p = match kind {
            GhostType::Blinky => Position::new(15, 15),
            GhostType::Pinky  => Position::new(15, 14),
            GhostType::Inky   => Position::new(14, 15),
            GhostType::Clyde  => Position::new(14, 14),
        };
        let rng_state = match kind {
            GhostType::Blinky => 0x1A2B_3C4D,
            GhostType::Pinky  => 0x5E6F_7A8B,
            GhostType::Inky   => 0x9CDE_F012,
            GhostType::Clyde  => 0x3456_7890,
        };
        Ghost {
            pos: start_p,
            last_pos: Position::new(i32::MIN, i32::MIN),
            house_timer: match kind {
                GhostType::Blinky => 2,
                GhostType::Pinky  => 10,
                GhostType::Inky   => 20,
                GhostType::Clyde  => 30,
            },
            dir: Direction::Up,
            rng_state,
            kind,
        }
    }

    pub fn get_position(&self) -> Position {
        self.pos
    }

    fn next_rand(&mut self) -> u32 {
        self.rng_state = self.rng_state
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        self.rng_state
    }

    pub fn update(&mut self, map: &Map, occupied: &[Position]) {
        if self.house_timer > 0 {
            self.house_timer -= 1;
            if self.house_timer == 0 {
                self.pos = Position::new(13, 11);
                self.dir = Direction::Left;
            }
            return;
        }

        let opposite = match self.dir {
            Direction::Up    => Direction::Down,
            Direction::Down  => Direction::Up,
            Direction::Left  => Direction::Right,
            Direction::Right => Direction::Left,
        };

        let all_dirs = [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ];

        let mut preferred: Vec<Direction> = Vec::new();
        let mut crowded: Vec<Direction> = Vec::new();

        for &d in &all_dirs {
            if d == opposite {
                continue;
            }

            let (nx, ny) = match d {
                Direction::Up    => (self.pos.x,     self.pos.y - 1),
                Direction::Down  => (self.pos.x,     self.pos.y + 1),
                Direction::Left  => (self.pos.x - 1, self.pos.y),
                Direction::Right => (self.pos.x + 1, self.pos.y),
            };
            let next_pos = Position::new(nx, ny);

            let passable = match map.get(&next_pos) {
                Some(Tile::Wall) | Some(Tile::Room) => false,
                Some(_) => true,
                None => ny == self.pos.y,
            };

            if passable {
                if occupied.iter().any(|o| o.x == nx && o.y == ny) {
                    crowded.push(d);
                } else {
                    preferred.push(d);
                }
            }
        }

        let choices = if !preferred.is_empty() {
            preferred
        } else if !crowded.is_empty() {
            crowded
        } else {
            vec![opposite]
        };

        let rnd = self.next_rand() as usize;
        self.dir = choices[rnd % choices.len()];

        let mut nx = self.pos.x;
        let mut ny = self.pos.y;
        match self.dir {
            Direction::Up    => ny -= 1,
            Direction::Down  => ny += 1,
            Direction::Left  => nx -= 1,
            Direction::Right => nx += 1,
        }

        if nx < 0 {
            nx = TILE_MAP_WIDTH as i32 - 1;
        } else if nx >= TILE_MAP_WIDTH as i32 {
            nx = 0;
        }

        self.last_pos = self.pos;
        self.pos = Position::new(nx, ny);
    }
}

pub struct GhostController {
    ghosts: [Ghost; 4],
    mode: GhostMode,
    timer: u16,
    num_scatters: u8,
}

impl GhostController {
    pub fn new() -> Self {
        Self {
            ghosts: [
                Ghost::new(GhostType::Blinky),
                Ghost::new(GhostType::Clyde),
                Ghost::new(GhostType::Inky),
                Ghost::new(GhostType::Pinky),
            ],
            mode: GhostMode::Pursue,
            timer: 0,
            num_scatters: 2,
        }
    }

    pub fn update(&mut self, map: &Map) {
        self.timer = self.timer.wrapping_add(1);

        let positions: [Position; 4] = [
            self.ghosts[0].pos,
            self.ghosts[1].pos,
            self.ghosts[2].pos,
            self.ghosts[3].pos,
        ];

        for (i, ghost) in self.ghosts.iter_mut().enumerate() {
            let others: Vec<Position> = positions
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, p)| *p)
                .collect();
            ghost.update(map, &others);
        }
    }

    // Hayaletlere ait diziyi verir. Referans olarak
    pub fn get(&self) -> &[Ghost] {
        &self.ghosts
    }
}
