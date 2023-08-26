use crate::common::{spawn_star, spawn_stars_full};
use crate::game::star::resources::StarSpawnTimer;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    spawn_stars_full(&mut commands, &window_query, &asset_server);
}

// Sistemdeki yıldızlar için bir timer nesnesi uygulanıyor
pub fn tick_star_spawn_timer(mut star_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_timer.timer.tick(time.delta());
}

pub fn spawn_star_after_time_finished(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_timer: Res<StarSpawnTimer>,
) {
    // star saati için belirlenen süre geçmişse
    if star_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        spawn_star(&mut commands, &asset_server, window);
    }
}
