use crate::command::Command;
use std::io;
use std::io::Write;

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
    let mut command = Command::default();
    command.parse(input.as_str());
    command
}
