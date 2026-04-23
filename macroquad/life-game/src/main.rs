use macroquad::prelude::*;
use std::fmt::{Display, Formatter};

const WINDOW_SIZE: f32 = 320.;
const CELL_SIZE: f32 = 4.;

fn window_conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_string(),
        fullscreen: false,
        window_resizable: false,
        window_width: WINDOW_SIZE as i32,
        window_height: WINDOW_SIZE as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut cells = Vec::new();
    let (row, col) = (
        (WINDOW_SIZE / CELL_SIZE) as i32,
        (WINDOW_SIZE / CELL_SIZE) as i32,
    );
    //println!("{} X {}", row, col);
    let mut counter: usize = 0;
    for i in 0..col {
        // println!("{}. satır", i);
        for j in 0..row {
            // println!("\t{} sütun", j);
            let x = j as f32 * CELL_SIZE;
            let y = i as f32 * CELL_SIZE;

            let state = match rand::gen_range(0, 7) {
                0 => CellState::Alive,
                _ => CellState::Dead,
            };
            let c = Cell::new(counter, Vec2::new(x, y), state);
            cells.push(c);
            counter += 1;
            //print!("{},", c);
        }
    }

    loop {
        clear_background(WHITE);

        for i in 0..cells.len() {
            let mut live_neighbors_count = 0;
            let id = cells[i].id;

            // Sağ komşu kontrolü
            if Vec2::new(cells[i].position.x + CELL_SIZE, cells[i].position.y).x < WINDOW_SIZE {
                if let CellState::Alive = cells[id + 1].state {
                    live_neighbors_count += 1;
                };

                // Sağ üst komşu kontrolü
                if Vec2::new(
                    cells[i].position.x + CELL_SIZE,
                    cells[i].position.y - (2. * CELL_SIZE),
                )
                .y >= 0.
                {
                    if let CellState::Alive = cells[(id + 1) - col as usize].state {
                        live_neighbors_count += 1;
                    };
                }
                // Sağ alt komşu kontrolü
                if Vec2::new(
                    cells[i].position.x + CELL_SIZE,
                    cells[i].position.y + (2. * CELL_SIZE),
                )
                .y < WINDOW_SIZE
                {
                    if let CellState::Alive = cells[(id + 1) + col as usize].state {
                        live_neighbors_count += 1;
                    };
                }
            }

            // Sol komşu kontrolü
            if Vec2::new(cells[i].position.x - CELL_SIZE, cells[i].position.y).x >= 0. {
                if let CellState::Alive = cells[id - 1].state {
                    live_neighbors_count += 1;
                };

                // Sol üst komşu kontrolü
                if Vec2::new(
                    cells[i].position.x - CELL_SIZE,
                    cells[i].position.y - (2. * CELL_SIZE),
                )
                .y >= 0.
                {
                    if let CellState::Alive = cells[(id - 1) - col as usize].state {
                        live_neighbors_count += 1;
                    };
                }

                // Sol alt komşu kontrolü
                if Vec2::new(
                    cells[i].position.x - CELL_SIZE,
                    cells[i].position.y + (2. * CELL_SIZE),
                )
                .y < WINDOW_SIZE
                {
                    if let CellState::Alive = cells[(id - 1) + col as usize].state {
                        live_neighbors_count += 1;
                    };
                }
            }

            // Alt komşu kontrolü
            if Vec2::new(cells[i].position.x, cells[i].position.y + CELL_SIZE).y < WINDOW_SIZE {
                if let CellState::Alive = cells[id + col as usize].state {
                    live_neighbors_count += 1;
                };
            }
            // Üst komşu kontrolü
            if Vec2::new(cells[i].position.x, cells[i].position.y - CELL_SIZE).y >= 0. {
                if let CellState::Alive = cells[id - col as usize].state {
                    live_neighbors_count += 1;
                };
            }

            match cells[i].state {
                CellState::Alive => {
                    if live_neighbors_count < 2 {
                        //println!("{} canlı komşu sayısı 2den az", id);
                        cells[i].state = CellState::Dead;
                    } else if live_neighbors_count == 2 || live_neighbors_count == 3 {
                        //println!("{} canlı komşu sayısı 2 veya 3", id);
                        cells[i].state = CellState::Alive
                    } else if live_neighbors_count > 3 {
                        //println!("{} canlı komşu sayısı 3den fazla", id);
                        cells[i].state = CellState::Dead;
                    } else {
                        //println!("{} hiç komşu yok gibi", id);
                    }
                }
                CellState::Dead => {
                    if live_neighbors_count == 3 {
                        //println!("{} ölü ama 3 canlı komşusu var", id);
                        cells[i].state = CellState::Alive
                    }
                }
            }
        }

        for c in &cells {
            c.draw();
        }

        next_frame().await
    }
}

#[derive(Copy, Clone)]
pub enum CellState {
    Alive,
    Dead,
}

impl Display for CellState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Alive => write!(f, "Canlı"),
            CellState::Dead => write!(f, "Ölü"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Cell {
    id: usize,
    state: CellState,
    position: Vec2,
}

impl Cell {
    pub fn new(id: usize, position: Vec2, state: CellState) -> Self {
        Self {
            id,
            state,
            position,
        }
    }
    pub fn draw(&self) {
        let color = match &self.state {
            CellState::Alive => BLACK,
            CellState::Dead => WHITE,
        };
        draw_rectangle(
            self.position.x,
            self.position.y,
            CELL_SIZE as f32,
            CELL_SIZE as f32,
            color,
        );
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.id, self.position, self.state)
    }
}
