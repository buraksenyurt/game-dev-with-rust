use std::io::{stdin, stdout, Read, Write};
pub fn get_input() -> Option<String> {
    let mut input = String::new();
    if stdin().read_line(&mut input).is_ok() {
        return Some(input.trim().to_string());
    }
    None
}

pub fn pause() {
    let mut stdin = stdin();
    let mut stdout = stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
