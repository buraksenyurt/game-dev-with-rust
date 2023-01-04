/*
   Bevy, ECS(Entity Component System) odaklı bir oyun motoru.
   Bağımlılıkların yönetimi nispeten daha kolay.
   ECS belli prensipler barındıran bir yazılım deseni gibi düşünülebilir.
   Örneğin Position ve Velocity birer component olarak düşünüldüğünde bu bileşenleri
   kullanan Player, Enemy gibi entity'lerden bahsedebiliriz.
   Movement gibi bir sistem tasarlayıp Position ve Velocity bileşenlerini içerek tüm entity'ler
   için işletebiliriz.

   Bevy modülerliğe önem veren bir oyun motoru. Bu nedenle tüm özellikler birer plugin olarak
   uyarlanır. Kendi plugin'lerimizi yazabiliriz de ama bevy birçok varsayılan plugin sağlar.
*/
use bevy::app::App;
use bevy::ecs::component::Component;
use bevy::prelude::{Commands, Plugin, Query, With};
use bevy::DefaultPlugins;

fn main() {
    // Default plugins var sayılan bazı plugin'ler ekler.
    // Standart bir pencere ve sonsuz oyun döngüsü bunlar arasındadır.
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FirstPlugin)
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

// Tasarladığımız add_players, first_system ve greetings_all sistemlerini
// bir plugin altında toplayabilir ve ana uygulama motoruna ekleyebiliriz.
pub struct FirstPlugin;

impl Plugin for FirstPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_players)
            .add_system(first_system)
            .add_system(greetings_for_all);
    }
}
