use inline_colorization::{color_bright_blue, color_reset};
use std::io::{stdin, stdout, Read, Write};

pub fn get_input() -> Option<String> {
    let mut input = String::new();
    if stdin().read_line(&mut input).is_ok() {
        return Some(input.trim().to_string());
    }
    None
}

pub fn pause(message: &str) {
    let mut stdin = stdin();
    let mut stdout = stdout();

    write!(stdout, "{color_bright_blue}{}{color_reset}", message).unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn check_team_name(input: &str) -> bool {
    input.len() > 5 || input.len() < 15
}
