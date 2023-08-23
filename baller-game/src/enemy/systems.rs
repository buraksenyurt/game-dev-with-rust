use super::components::Enemy;
use super::resources::EnemySpawnTimer;
use super::{ENEMEY_SPEED, ENEMY_SIZE};
use crate::common::{spawn_enemies_full, spawn_enemy};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// Enemy olarak adledilen kırmızı topların üretildiği sistem
pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    spawn_enemies_full(&mut commands, &window_query, &asset_server);
}

// Kırmızı topları(enemy) hareket ettiren sistem.
// Bu sefer Enemy olan bileşenlerin transform özelliklerini değiştirmemiz gerekiyor.
pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    // Ortamdaki enemy, transform bileşen eşleri mutable olarak dolaşılır
    for (mut transform, enemy) in enemy_query.iter_mut() {
        // Enemy spawn'lanırken bir direction verilmişti. Buna göre 3 boyutlu vektör örneklenir
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        // translation bilgisi ENEMY_SPEED ve delta time değerleri kullanılarak değiştirilir.
        transform.translation += direction * ENEMEY_SPEED * time.delta_seconds();
    }
}

pub fn enemy_out_of_bound_check(
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

pub fn check_enemy_movement(
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

// Sistemdeki kırmızı toplar için bir timer nesnesi uygulanıyor
pub fn enemy_spawn_timer(mut star_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    star_timer.timer.tick(time.delta());
}

pub fn spawn_enemy_after_time_finished(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_timer: Res<EnemySpawnTimer>,
) {
    // enemy için belirlenen süre geçmişse
    if enemy_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        spawn_enemy(&mut commands, &asset_server, window);
    }
}
