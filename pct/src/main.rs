mod constants;
mod game;
mod pacman;

use crate::constants::*;
use crate::game::Game;
use crate::pacman::Pacman;
use std::io::{self};
use std::thread;
use std::time::Duration;

fn main() {
    let mut board = vec![vec!['.'; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut pacman = Pacman {
        x: SCREEN_WIDTH / 2,
        y: SCREEN_HEIGHT / 2,
    };
    board[pacman.y][pacman.x] = 'P'; //'\u{1F980}';

    loop {
        Game::print_board(&board);

        println!("Hareket etmek için WASD: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_uppercase();

        match input.as_str() {
            "W" => pacman.y = pacman.y.saturating_sub(1),
            "A" => pacman.x = pacman.x.saturating_sub(1),
            "S" => {
                if pacman.y < SCREEN_HEIGHT - 1 {
                    pacman.y += 1
                }
            }
            "D" => {
                if pacman.x < SCREEN_WIDTH - 1 {
                    pacman.x += 1
                }
            }
            _ => {}
        }

        Game::clear_screen();

        board = vec![vec!['.'; SCREEN_WIDTH]; SCREEN_HEIGHT];
        board[pacman.y][pacman.x] = 'P'; //'\u{1F980}';

        thread::sleep(Duration::from_millis(100));
    }
}
