use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub const PLAYER_SIZE: f32 = 64.;
pub const PLAYER_SPEED: f32 = 500.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera_system, spawn_player_system))
        .add_systems(
            Update,
            (player_movement_system, check_player_movement_system),
        )
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

// Oyuncunun hareket ettirildiği sistem
// Klavye yön tuşlarına göre hareket edeceğinden Input<KeyCode> bir Resource olarak alınır
// Hareket hesaplamasında delta time kullanılacağından Time verisi de Resource olarak alınır.
// Tabi Transform bileşenlerinden oyuncu Entity'si ile yüklenenler sorgulanır
pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    // Eğer ortamda hareket edebilecek bir player nesnesi varsa
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        // Basılan tuşa göre yeni bir Vektör hareketi ayarlanır
        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1., 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1., 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0., 1., 0.);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0., -1., 0.);
        }

        // Bir hareket söz konusu ise vektör birim vektöre çevirilir(Normalize)
        if direction.length() > 0. {
            direction = direction.normalize();
        }

        // bileşenin konumu direction, player_speed ve delta time süresine göre tekrardan ayarlanır.
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn check_player_movement_system(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let (x_min, x_max) = (PLAYER_SIZE / 2., window.width() - PLAYER_SIZE / 2.);
        let (y_min, y_max) = (PLAYER_SIZE / 2., window.height() - PLAYER_SIZE / 2.);
        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}
