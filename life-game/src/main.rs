use macroquad::prelude::*;
use std::fmt::{Display, Formatter};

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;

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
    for i in 0..20 {
        for j in 0..15 {
            let state = match rand::gen_range(0, 5) {
                0 => CellState::Alive,
                _ => CellState::Dead,
            };
            let c = Cell::new(Vec2::new((i * 32) as f32, (j * 32) as f32), state);
            let _ = &cells.push(c);
            println!("{}", c);
        }
    }
    loop {
        clear_background(WHITE);

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
    state: CellState,
    position: Vec2,
}

impl Cell {
    pub fn new(position: Vec2, state: CellState) -> Self {
        Self { state, position }
    }
    pub fn draw(&self) {
        let color = match self.state {
            CellState::Alive => BLACK,
            CellState::Dead => WHITE,
        };
        draw_rectangle(self.position.x, self.position.y, 32., 32., color);
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.position, self.state)
    }
}
