use crate::command::Command;
use crate::controller::{update_screen, update_state};

mod command;
mod controller;

fn main() {
    println!("Gizeme hoş geldin. Bu bir eski zaman macerası.\n");
    println!("Üstün başın dağılmış bir halde ve korkunç bir baş ağrısıyla uyandın.");
    println!("Dışarıda yağmur yağdığını hayal mayal duyuyor gibisin ama baş ağrın korkunç.");
    println!("Gözlerini açmaya çalıştığın sırada telefon çalıyor.\n");

    let mut command = Command::default();
    let mut output: String;
    loop {
        command = controller::take_input();

        if command.verb == "bitir" {
            break;
        }
        output = update_state(&command);
        update_screen(output);
    }

    println!("Görüşmek üzere!");
}
