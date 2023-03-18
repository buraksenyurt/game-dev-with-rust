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

// Oyuncunun girdiği komuta göre state güncellemesi için kullanılacak yardımcı fonksiyon
pub fn update_state(command: &Command) -> String {
    match command.verb.as_str() {
        "bak" => "Etraf çok karanlık. Telefonu bulabilecek misin?".to_string(),
        "kalk" => {
            "Bu zifiri karanlıkta odada ilerlemek zor olabilir. Eğer gerçekten bir odadaysan."
                .to_string()
        }
        "yokla" => "Etrafı kolaçan etmek iyi fikir. Ellerini kullan.".to_string(),
        "bitir" => "Kapatılıyor. Oynadığın için teşekkürler.".to_string(),
        _ => "Söylediğini nasıl yapabiliriz bilemiyorum.".to_string(),
    }
}

pub fn update_screen(output: String) {
    println!("{}", output)
}
