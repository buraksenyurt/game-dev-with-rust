use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (
                count_actors_system,
                count_only_soldiers,
                count_not_soldiers,
                count_soldiers,
            ),
        )
        .run()
}
pub fn setup_system(mut commands: Commands) {
    // Commands nesnesini entity oluşturmak, yok etmek
    // entity'lere component eklemek ve çıkarmak için kullanırız.

    // Bir entity oluştururken birden fazla component' de verebiliriz.
    // Rambo bir askerdir.
    commands.spawn((
        Actor {
            name: "Rambo".to_string(),
        },
        Soldier { class: Class::Army },
    ));

    // Lincoln Eco Six bir asker olarak tanımlanmamıştır
    // Nitekim Soldier component'ine sahip değildir.
    commands.spawn(Actor {
        name: "Lincoln Eco Six".to_string(),
    });

    commands.spawn((
        Actor {
            name: "Bevy Soldier".to_string(),
        },
        Soldier {
            class: Class::GreenBeret,
        },
    ));
}

// query ile sistemdeki Actor entity nesnelerini sorgulayabiliriz
pub fn count_actors_system(query: Query<&Actor>) {
    for actor in query.iter() {
        println!("'{}' is ready to serve", actor.name)
    }
}

// Sisteme bazı aktörleri yükledik. Bunlardan sadece Soldier
// component'i olanları sorgulayabiliriz.
// Actors with Soldier components
pub fn count_only_soldiers(query: Query<&Actor, With<Soldier>>) {
    for actor in query.iter() {
        println!("{} is soldier", actor.name)
    }
}

// Sisteme bazı aktörleri yükledik. Bunlardan asker olmayanları da
// yani Soldier component'ine sahip olmayanları da çekebiliriz.
// Actors without Soldier components
pub fn count_not_soldiers(query: Query<&Actor, Without<Soldier>>) {
    for actor in query.iter() {
        println!("{} is soldier", actor.name)
    }
}

// İstersek n sayıda component'i de bir arada sorgulayarak kullanan sistemler tasarlayabiliriz.
// Anahtar nokta Query veri modelinde ilgili bileşenleri Tuple içerisinde ele almaktır.
pub fn count_soldiers(query: Query<(&Actor, &Soldier)>) {
    for (actor, soldier) in query.iter() {
        let soldier_class = match soldier.class {
            Class::Marine => "Marine Corp",
            Class::Army => "Army",
            Class::Seal => "Seal",
            Class::Commando => "Commando",
            Class::GreenBeret => "Green Beret",
        };
        println!("{} professions is {}", actor.name, soldier_class);
    }
}

#[derive(Component)]
pub struct Actor {
    pub name: String,
}

#[derive(Component)]
pub struct Soldier {
    pub class: Class,
}

#[derive(Debug)]
pub enum Class {
    Marine,
    Army,
    Seal,
    Commando,
    GreenBeret,
}
