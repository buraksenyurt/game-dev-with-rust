mod constants;
mod game;
mod ghost;
mod pacman;

use crate::constants::*;
use crate::game::Game;
use crate::ghost::Ghost;
use crate::pacman::Pacman;
use std::io::{self};

fn main() {
    let mut board = vec![vec!['.'; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut pacman = Pacman {
        x: SCREEN_WIDTH / 2,
        y: SCREEN_HEIGHT / 2,
    };
    let mut ghost = Ghost {
        x: SCREEN_WIDTH / 4,
        y: SCREEN_HEIGHT / 4,
    };

    loop {
        Game::print_board(&board, pacman, ghost);

        println!("WASD: ");
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

        Game::move_to_player(&mut pacman, &mut ghost);

        if Game::check_game_over(&mut pacman, &mut ghost) {
            break;
        }

        Game::clear_screen();
    }
}
