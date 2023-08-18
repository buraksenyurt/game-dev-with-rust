use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera_system, spawn_player_system))
        .run();
}

#[derive(Component)]
pub struct Player {}

// AssetServer bir Resource'dur.
pub fn spawn_player_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    // Mavi topu ekranın tam ortasına yerleştirmek için pencerenin ölçüleri gerekcektir
    // Bu nedenle PrimaryWindows componentini taşıyan window nesnesi sorgulanır

    // Bir Entity nesnesine birden fazla component eklemek veya çıkarmak için Bundle'lar kullanılır.
    // Mavi topu konum bilgisi ile yükleyen bileşenleri Player ile ilişkilendirdik.
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Kameranın konumunu ayarlamak için yine ekran bilgisine ihtiyacımız olacak
    let window = window_query.get_single().unwrap();

    // 2D kamerayı da mavi topun olduğu koordinata göre konumlandırıyoruz
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    });
}
