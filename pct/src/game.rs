use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ghost::Ghost;
use crate::pacman::Pacman;
use std::process::Command;

pub struct Game;

impl Game {
    pub fn print_board(board: &Vec<Vec<char>>, pacman: Pacman, ghost: Ghost) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if x == pacman.x && y == pacman.y {
                    print!("P");
                } else if x == ghost.x && y == ghost.y {
                    print!("G");
                } else {
                    print!("{}", board[y][x]);
                }
            }
            println!();
        }
    }

    pub fn clear_screen() {
        let _ = Command::new("clear").status();
    }

    pub fn check_game_over(pacman: &mut Pacman, ghost: &mut Ghost) -> bool {
        if ghost.x == pacman.x && ghost.y == pacman.y {
            println!("You've been caught by the ghost!");
            return true;
        }
        false
    }

    pub fn move_to_player(pacman: &mut Pacman, ghost: &mut Ghost) {
        ghost.x = if ghost.x > pacman.x {
            ghost.x - 1
        } else if ghost.x < pacman.x {
            ghost.x + 1
        } else {
            ghost.x
        };
        ghost.y = if ghost.y > pacman.y {
            ghost.y - 1
        } else if ghost.y < pacman.y {
            ghost.y + 1
        } else {
            ghost.y
        };
    }
}
