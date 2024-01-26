use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ghost::Ghost;
use crate::pacman::Pacman;
use std::cmp::Ordering;
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
        let x_diff = if pacman.x > ghost.x {
            pacman.x - ghost.x
        } else {
            ghost.x - pacman.x
        };
        let y_diff = if pacman.y > ghost.y {
            pacman.y - ghost.y
        } else {
            ghost.y - pacman.y
        };

        if x_diff > y_diff {
            if pacman.x > ghost.x {
                ghost.x += 1;
            } else if pacman.x < ghost.x {
                ghost.x -= 1;
            }
        } else {
            if pacman.y > ghost.y {
                ghost.y += 1;
            } else if pacman.y < ghost.y {
                ghost.y -= 1;
            }
        }
    }
}
