use crate::command::Command;
use log::info;
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
    parse(input.as_str())
}

// girilen metni iki parça olarak ele alır ve Command nesnesine dönüştürüp geri döndürür
pub fn parse(input: &str) -> Command {
    let lovercase = input.to_lowercase();
    let mut parts = lovercase.trim().split_whitespace();
    let noun = parts.next().unwrap_or_default().to_string();
    let verb = parts.next().unwrap_or_default().to_string();
    info!("Parse işlemi. Noun : {} Verb : {}", noun, verb);

    match verb.as_str() {
        "bak" => Command::Look(noun),
        "git" => Command::Jump(noun),
        "kalk" => Command::GetUp,
        "bitir" => Command::Quit,
        _ => Command::Unknown(input.trim().to_string()),
    }
}

pub fn update_screen(output: String) {
    println!("{}", output)
}
