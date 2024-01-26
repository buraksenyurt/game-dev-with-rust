use std::process::Command;

pub struct Game;

impl Game {
    pub fn print_board(board: &Vec<Vec<char>>) {
        for row in board {
            for &cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }

    pub fn clear_screen() {
        let _ = Command::new("clear").status();
    }
}
