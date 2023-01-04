/*
   Bevy, ECS(Entity Component System) odaklı bir oyun motoru.
   Bağımlılıkların yönetimi nispeten daha kolay.
   ECS belli prensipler barındıran bir yazılım deseni gibi düşünülebilir.
   Örneğin Position ve Velocity birer component olarak düşünüldüğünde bu bileşenleri
   kullanan Player, Enemy gibi entity'lerden bahsedebiliriz.
   Movement gibi bir sistem tasarlayıp Position ve Velocity bileşenlerini içerek tüm entity'ler
   için işletebiliriz.
*/
use bevy::app::App;
use bevy::ecs::component::Component;
use bevy::prelude::{Commands, Query, With};

fn main() {
    App::new()
        .add_startup_system(add_players)
        .add_system(first_system)
        .add_system(greetings_for_all)
        .run();
}

fn first_system() {
    println!("First contact with first system :P");
}

// Oyuncu eklemek için başka bir sistem
fn add_players(mut commands: Commands) {
    commands.spawn((
        Player,
        Title("Teğmen Garsiya Lopez Dela Fuente".to_string()),
    ));
    commands.spawn((Player, Title("Kıdemli Yüzbaşı Korkmaz Gizmo".to_string())));
    commands.spawn((Player, Title("Başçavuş Stark".to_string())));
}

// Oyuncuları selamlayan bir sistem fonksiyonu daha
// Parametre şunu diyor; bu sistem tüm Title ve Player entity'leri için çalışır.
fn greetings_for_all(query: Query<&Title, With<Player>>) {
    for title in query.iter() {
        println!("Hoş geldin {}!", title.0);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Title(String);