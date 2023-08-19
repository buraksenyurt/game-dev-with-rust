use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

pub const PLAYER_SIZE: f32 = 64.;
pub const PLAYER_SPEED: f32 = 500.;
pub const ENEMY_SIZE: f32 = 64.;
pub const ENEMEY_SPEED: f32 = 200.;
pub const NUMBER_OF_ENEMIES: usize = 8;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                spawn_camera_system,
                spawn_player_system,
                spawn_enemies_system,
            ),
        )
        .add_systems(
            Update,
            (
                player_movement_system,
                check_player_movement_system,
                enemy_movement_system,
                enemy_bounces_system,
                check_enemy_movement_system,
                enemy_hit_player_system,
                refresh_enemies_system,
            ),
        )
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    direction: Vec2, // Kırmızı topların anlık konumlarını saklamak için eklendi
}

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

// Enemy olarak adledilen kırmızı topların üretildiği sistem
pub fn spawn_enemies_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // Bu sefer rastgele x,y konumlarında 8 adet düşman nesnesi örneklenmekte
    for _ in 0..NUMBER_OF_ENEMIES {
        let x = random::<f32>() * window.width();
        let y = random::<f32>() * window.height();

        // Kırmızı top üretilirken rastgele bir konuma konur ve ayrıca,
        // rastgele x,y değerlerini baz olan bir yöne gidecek şekilde ayarlanır.
        // Direction değerini işaret eden vektörün birim vektöre dönüştürüldüğüne dikkat.
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

// Kırmızı topları(enemy) hareket ettiren sistem.
// Bu sefer Enemy olan bileşenlerin transform özelliklerini değiştirmemiz gerekiyor.
pub fn enemy_movement_system(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    // Ortamdaki enemy, transform bileşen eşleri mutable olarak dolaşılır
    for (mut transform, enemy) in enemy_query.iter_mut() {
        // Enemy spawn'lanırken bir direction verilmişti. Buna göre 3 boyutlu vektör örneklenir
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        // translation bilgisi ENEMY_SPEED ve delta time değerleri kullanılarak değiştirilir.
        transform.translation += direction * ENEMEY_SPEED * time.delta_seconds();
    }
}

pub fn enemy_bounces_system(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Kırmızı topun sınır x,y değerleri bulunur
    let window = window_query.get_single().unwrap();
    let (x_min, x_max) = (ENEMY_SIZE / 2., window.width() - ENEMY_SIZE / 2.);
    let (y_min, y_max) = (ENEMY_SIZE / 2., window.height() - ENEMY_SIZE / 2.);
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        // X ekseninde sınırlara gelindiyse enemy bileşeninin x yönü tersine çevrilir
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.;
        }
        // y ekseninde sınırlara gelindiyse enemy bileşeninin y yönü tersine çevrilir
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.;
        }
    }
}

pub fn check_enemy_movement_system(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let x_min = ENEMY_SIZE / 2.;
    let x_max = window.width() - ENEMY_SIZE / 2.;
    let y_min = ENEMY_SIZE / 2.;
    let y_max = window.height() - ENEMY_SIZE / 2.;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

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
        transform.translation = translation;
    }
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

// Mavi topun kırmızı toplara çarptığı durumları ele alan sistem.
// Oyuncuyu temsil eden mavi topun bir kırmızı topa çarpması oyunun bitmesi anlamına da geliyor.
pub fn enemy_hit_player_system(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    // Sahadaki oyuncuyu transform bileşeni ile birlikte ele alıyoruz
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        // tüm kırmızı toplar için bir iterasyon var ve transform bileşenlerini ele alıyoruz
        for enemy_transform in enemy_query.iter() {
            // çarpışma kontrolü yapılıyor.
            // toplar birer daire olduğundan aralarındaki mesafenin yarıçapları toplamı ile olan
            // mukayesesi sayesinde çarpışma olup olmadığını anlayabiliriz.
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.;
            let enemy_radius = ENEMY_SIZE / 2.;
            if distance < player_radius + enemy_radius {
                info!("Game over!Catched with red ball");
                // Oyuncunun entity nesnesi sistemden kaldırılıyor
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn refresh_enemies_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut enemies_query: Query<Entity, With<Enemy>>,
    player_query: Query<Entity, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if keyboard_input.pressed(KeyCode::F5) {
        for enemy in enemies_query.iter_mut() {
            commands.entity(enemy).despawn();
        }
        if let Ok(player) = player_query.get_single() {
            commands.entity(player).despawn();
        }
        let window = window_query.get_single().unwrap();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Player {},
        ));
        spawn_enemies_system(commands, window_query, asset_server);
    }
}
