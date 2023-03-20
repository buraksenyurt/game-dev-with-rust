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
    parse(input.as_str())
}

// girilen metni iki parça olarak ele alır ve Command nesnesine dönüştürüp geri döndürür
pub fn parse(input: &str) -> Command {
    let lovercase = input.to_lowercase();
    let mut parts = lovercase.trim().split_whitespace();
    let noun = parts.next().unwrap_or_default().to_string();
    let verb = parts.next().unwrap_or_default().to_string();

    match verb.as_str() {
        "bak" => Command::Look(noun),
        "yürü" => Command::Walk(noun),
        "kalk" => Command::GetUp(noun),
        "bitir" => Command::Quit,
        _ => Command::Unknown(input.trim().to_string()),
    }
}

// Oyuncunun girdiği komuta göre state güncellemesi için kullanılacak yardımcı fonksiyon
pub fn update_state(command: &Command) -> String {
    match command {
        Command::Walk(_) => "Etraf çok karanlık. Telefonu bulabilecek misin?".to_string(),
        Command::GetUp(_) => {
            "Bu zifiri karanlıkta odada ilerlemek zor olabilir. Eğer gerçekten bir odadaysan."
                .to_string()
        }
        Command::Look(_) => "Etrafı kolaçan etmek iyi fikir. Ellerini kullan.".to_string(),
        Command::Quit => "Kapatılıyor. Oynadığın için teşekkürler.".to_string(),
        Command::Unknown(input) => format!("{} Söylediğini nasıl yapabiliriz bilemiyorum.", input),
    }
}

pub fn update_screen(output: String) {
    println!("{}", output)
}
