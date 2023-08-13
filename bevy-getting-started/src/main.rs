use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ActorPlugin))
        // .add_systems(Startup, setup_system)
        // .add_systems(Update, (commando_hello_system, enemy_hello_system))
        .run();
}

struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_system)
            .add_systems(Update, (commando_hello_system, enemy_hello_system));
    }
}

fn setup_system(mut commands: Commands) {
    commands.spawn((Commando, Name("Can Cey Rambo".to_string())));
    commands.spawn((Enemy, Name("Ays Men".to_string())));
    commands.spawn((Enemy, Name("Raven".to_string())));
    commands.spawn((Enemy, Name("The Snake".to_string())));
    commands.spawn((Bunker, Name("North Shield".to_string()), Capacity(4)));
}

fn commando_hello_system(query: Query<&Name, With<Commando>>) {
    for n in &query {
        info!("{} sahada", n.0);
    }
}

fn enemy_hello_system(query: Query<&Name, With<Enemy>>) {
    for n in &query {
        info!("Düşman kuvvetlerden {} sahada", n.0);
    }
}

#[derive(Component)]
struct Commando;

// Farklı entity'lerin de isimleri olabileceğinden
// Name diye ayrı bir component söz konusu
#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Bunker;

// Bir sığınağın bir aracın personel kapasitesi olabilir
// Bunu da ayrı bir component olarak tasarlayabiliriz
#[derive(Component)]
struct Capacity(u8);
