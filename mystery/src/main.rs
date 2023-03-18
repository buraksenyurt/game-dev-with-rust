use crate::command::Command;

mod command;
mod controller;

fn main() {
    println!("Gizeme hoş geldin. Bu bir eski zaman macerası.\n");
    println!("Üstün başın dağılmış bir halde ve korkunç bir baş ağrısıyla uyandın.");
    println!("Dışarıda yağmur yağdığını hayal mayal duyuyor gibisin ama baş ağrın korkunç.\n");

    let mut command = Command::default();

    loop {
        command = controller::take_input();

        if command.noun == "Bitir" {
            break;
        }
        println!("{}", command);
    }

    println!("Görüşmek üzere!");
}
