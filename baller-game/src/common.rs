use crate::enemy::components::Enemy;
use crate::enemy::NUMBER_OF_ENEMIES;
use crate::star::components::Star;
use crate::star::NUMBER_OF_STARS;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

pub fn spawn_stars_full(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_STARS {
        spawn_star(commands, asset_server, window);
    }
}

pub fn spawn_star(commands: &mut Commands, asset_server: &Res<AssetServer>, window: &Window) {
    let x = random::<f32>() * window.width();
    let y = random::<f32>() * window.height();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.),
            texture: asset_server.load("sprites/star.png"),
            ..default()
        },
        Star {},
    ));
}

pub fn spawn_enemies_full(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // Bu sefer rastgele x,y konumlarında 6 adet düşman nesnesi örneklenmekte
    for _ in 0..NUMBER_OF_ENEMIES {
        spawn_enemy(commands, asset_server, window);
    }
}

pub fn spawn_enemy(commands: &mut Commands, asset_server: &Res<AssetServer>, window: &Window) {
    let x = random::<f32>() * window.width();
    let y = random::<f32>() * window.height();

    let direction = Vec2::new(random::<f32>(), random::<f32>()).normalize();
    info!("Direction {:?}", direction);
    // Kırmızı top üretilirken rastgele bir konuma konur ve ayrıca,
    // rastgele x,y değerlerini baz olan bir yöne gidecek şekilde ayarlanır.
    // Direction değerini işaret eden vektörün birim vektöre dönüştürüldüğüne dikkat.
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.),
            texture: asset_server.load("sprites/ball_red_large.png"),
            ..default()
        },
        Enemy { direction },
    ));
}
