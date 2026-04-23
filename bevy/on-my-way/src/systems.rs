use crate::events::GameOverEvent;
use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

pub const STARS_COUNT: u8 = 200;
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
    ));
}
pub fn handle_game_over(
    mut event_reader: EventReader<GameOverEvent>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for event in event_reader.read() {
        info!("Oyun sonlandı...Oyuncunun skoru {}", event.current_score);
        app_state.set(AppState::GameOver);
    }
}

pub fn change_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::F5) && app_state.0 != Option::from(AppState::Game) {
        app_state.set(AppState::Game);
        info!("'Game' modunda geçildi...");
    }
}

pub fn change_to_main_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::F2) && app_state.0 != Option::from(AppState::MainMenu) {
        app_state.set(AppState::MainMenu);
        info!("'Main Menu' moduna geçildi...");
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();
    let star_sizes = [0.25, 1., 1.25, 1.5, 1.75, 2.];
    let mut rng = rand::rng();
    for _ in 0..STARS_COUNT {
        let idx = rng.random_range(0..star_sizes.len());
        let (x, y) = (
            rng.random_range(0.0..window.width()),
            rng.random_range(0.0..window.height()),
        );
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(star_sizes[idx]))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.98, 0.92, 0.84)))),
            Transform::from_translation(Vec3::new(x, y, 0.)),
        ));
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        event_writer.send(AppExit::Success);
    }
}
