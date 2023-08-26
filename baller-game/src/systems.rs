use crate::common::{spawn_enemies_full, spawn_stars_full};
use crate::events::*;
use crate::game::enemy::components::Enemy;
use crate::game::player::components::Player;
use crate::game::score::resources::Score;
use crate::game::star::components::Star;
use crate::game::PlayGroundState;
use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    // Kameranın konumunu ayarlamak için yine ekran bilgisine ihtiyacımız olacak
    let window = window_query.get_single().unwrap();

    // 2D kamerayı da mavi topun olduğu koordinata göre konumlandırıyoruz
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    });
}

pub fn refresh_everything(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut enemies_query: Query<Entity, With<Enemy>>,
    mut stars_query: Query<Entity, With<Star>>,
    player_query: Query<Entity, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if keyboard_input.pressed(KeyCode::F5) {
        score.value = 0u32;
        for enemy in enemies_query.iter_mut() {
            commands.entity(enemy).despawn();
        }
        for star in stars_query.iter_mut() {
            commands.entity(star).despawn();
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
        spawn_enemies_full(&mut commands, &window_query, &asset_server);
        spawn_stars_full(&mut commands, &window_query, &asset_server);
    }
}

// ESC tuşuna basıldığında oyundan çıkılması için kullanılan sistemdir.
// EventWriter ile başka bir sisteme event bildirimi yapabiliriz.
// Buna benzer şekilde EventReader ile de sisteme gelen event'leri yakalayabiliriz.
// Bevy tarafında event'ler sistemler arasında veri taşımak için kullanılır.
// Böylece bir sistemde meydana gelen bir aksiyon karşılığında bir event verisi oluşturup
// bunu kullanacak başka bir sisteme yollayabiliriz. Cool!
pub fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        event_writer.send(AppExit);
    }
}

// GameOver isimli bir event oluştuğunda ele alınan sistem
pub fn handle_game_over(mut commands: Commands, mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        info!("Player's final score is {}", event.final_score.to_string());
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}

pub fn change_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if **app_state != AppState::Game {
            commands.insert_resource(NextState(Some(AppState::Game)));
            info!("Now in 'Game' state");
        }
    }
}

pub fn change_to_main_menu(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if **app_state != AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
            commands.insert_resource(NextState(Some(PlayGroundState::Paused)));
            info!("Now in 'Main Menu' state");
        }
    }
}
