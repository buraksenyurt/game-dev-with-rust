use crate::command::Command;
use crate::controller::update_screen;
use crate::game_world::GameWorld;

mod command;
mod controller;
mod game_world;
mod location;
mod object;
mod player;

fn main() {
    env_logger::init();

    println!("Gizeme hoş geldin. Bu bir eski ve belki de henüz yaşanmamış bir macera.\n");
    println!("Üstün başın dağılmış bir halde ve korkunç bir baş ağrısıyla uyandın.");
    println!("Dışarıda yağmur yağdığını hayal mayal duyuyor gibisin ama baş ağrın korkunç.");
    println!("Gözlerini açmaya çalıştığın sırada yakınlardan gelen bir alarm sesi duyuyorsun.\n");

    let mut output: String;
    let mut command: Command;
    let mut game_world: GameWorld = GameWorld::default();
    loop {
        command = controller::take_input();

        if matches!(command, Command::Quit) {
            break;
        }
        output = game_world.update_state(&command);
        update_screen(output);
    }

    println!("Görüşmek üzere!");
}
