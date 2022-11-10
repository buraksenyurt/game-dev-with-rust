use macroquad::prelude::*;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let game = Game::new(screen_width() as usize, screen_height() as usize);
    loop {
        clear_background(WHITE);

        next_frame().await
    }
}

enum CellState {
    Alive,
    Dead,
}

struct Game {
    cells: Vec<CellState>,
    buffer: Vec<CellState>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            cells: vec![CellState::Dead; size],
            buffer: vec![CellState::Dead; size],
        }
    }
}
