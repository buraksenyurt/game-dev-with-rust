use bevy::app::App;

fn main() {
    App::new().add_system(first_system).run();
}

fn first_system() {
    println!("First contact with first system :P");
}
