use super::{PLAYER_SIZE, PLAYER_SPEED};
use crate::events::GameOver;
use crate::game::enemy::components::Enemy;
use crate::game::enemy::ENEMY_SIZE;
use crate::game::player::components::Player;
use crate::game::score::resources::Score;
use crate::game::star::components::Star;
use crate::game::star::STAR_SIZE;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_player(
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

// Oyuncunun hareket ettirildiği sistem
// Klavye yön tuşlarına göre hareket edeceğinden Input<KeyCode> bir Resource olarak alınır
// Hareket hesaplamasında delta time kullanılacağından Time verisi de Resource olarak alınır.
// Tabi Transform bileşenlerinden oyuncu Entity'si ile yüklenenler sorgulanır
pub fn control_player_movement(
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

pub fn check_player_movement(
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
pub fn check_enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut game_over_event_writer: EventWriter<GameOver>,
    score: Res<Score>,
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
                // Oyuncu kırmız toplardan birisine yakalnırsa
                // bu sistemden GameOver isimli bir event fırlatıyoruz.
                // Bunu handle_game_over_system isimli sistem dinliyor olacak.
                // Ayrıca event ile birlikte güncel skor değerini de taşımaktayız.
                // Böylece bu değeri Reader event içinden okuyabiliriz
                game_over_event_writer.send(GameOver {
                    final_score: score.value,
                });
            }
        }
    }
}

pub fn check_player_hits_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transformation) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transformation.translation);
            let player_radius = PLAYER_SIZE / 2.;
            let star_radius = STAR_SIZE / 2.;
            if distance < player_radius + star_radius {
                score.value += 10;
                // info!(
                //     "Player catch a start! Current Score {}",
                //     score.value.to_string()
                // );
                commands.entity(star_entity).despawn();
            }
        }
    }
}
