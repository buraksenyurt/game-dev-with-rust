use crate::command::Command;
use std::io;
use std::io::Write;
use std::str::FromStr;

// Terminalden girdi alıp bunu Command nesnesine çeviren fonksiyon
pub fn take_input() -> Command {
    println!();
    print!("-> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Ne demek istediğini anlayamadım. Üzgünüm.");
    println!();
    Command::from_str(&input).unwrap()
}

pub fn update_screen(output: String) {
    println!("{}", output)
}
