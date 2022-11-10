use macroquad::prelude::*;
use std::fmt::{Display, Formatter};

const WINDOW_WIDTH: i32 = 320;
const WINDOW_HEIGHT: i32 = 320;
const CELL_SIZE: i32 = 32;

fn window_conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_string(),
        fullscreen: false,
        window_resizable: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut cells = Vec::new();
    let (row, col) = (WINDOW_HEIGHT / CELL_SIZE, WINDOW_WIDTH / CELL_SIZE);
    let mut index = 0;
    for i in 0..row {
        for j in 0..col {
            index += 1;
            let state = match rand::gen_range(0, 10) {
                0 => CellState::Alive,
                _ => CellState::Dead,
            };
            let c = Cell::new(
                index,
                Vec2::new((i * CELL_SIZE) as f32, (j * CELL_SIZE) as f32),
                state,
            );
            let _ = &cells.push(c);
            //print!("{},", c);
        }
    }
    loop {
        clear_background(WHITE);

        for i in 0..99 {
            let c = &cells[i];
            let mut current_id = c.id;
            let mut live_neighbors_count = 0;
            let mut dead_neighbors_count = 0;
            println!("{}", current_id);

            if current_id % col == 0 || current_id % col < (col - 1) {
                match cells[(current_id + 1) as usize].state {
                    CellState::Alive => live_neighbors_count += 1,
                    CellState::Dead => dead_neighbors_count += 1,
                }
            }
            if (current_id - 1) % col == 0 || (current_id - 1) % col < (col - 1) {
                match cells[(current_id - 1) as usize].state {
                    CellState::Alive => live_neighbors_count += 1,
                    CellState::Dead => dead_neighbors_count += 1,
                }
            }

            match cells.get((current_id - col) as usize) {
                Some(_) => match cells[(current_id - col) as usize].state {
                    CellState::Alive => live_neighbors_count += 1,
                    CellState::Dead => dead_neighbors_count += 1,
                },
                _ => {}
            };
            match cells.get((current_id + col) as usize) {
                Some(_) => match cells[(current_id + col) as usize].state {
                    CellState::Alive => live_neighbors_count += 1,
                    CellState::Dead => dead_neighbors_count += 1,
                },
                _ => {}
            }

            match c.state {
                CellState::Alive => {
                    if (live_neighbors_count == 0) {
                        println!("Hücre canlı ama hiç komşusu yok");
                    } else if (live_neighbors_count < 2) {
                        println!("Hücre canlı ve 2 den az canlı komşusu var");
                    } else if (live_neighbors_count == 2 || live_neighbors_count == 3) {
                        println!("Hücre canlı ve 2 veya 3 canlı komşusu var");
                    } else if (live_neighbors_count > 3) {
                        println!("Hücre canlı ve 3 ten fazla komşusu var");
                    }
                }
                CellState::Dead => {
                    if (live_neighbors_count == 3) {
                        println!("Hücre ölü ama 3 canlı komşusu var");
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

#[derive(Copy, Clone, PartialEq)]
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
    id: i32,
    state: CellState,
    position: Vec2,
}

impl Cell {
    pub fn new(id: i32, position: Vec2, state: CellState) -> Self {
        Self {
            id,
            state,
            position,
        }
    }
    pub fn draw(&self) {
        let color = match self.state {
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
        write!(f, "{}-> {}: {}", self.id, self.position, self.state)
    }
}
