use crate::events::GameOverEvent;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use rand::Rng;

pub const STARS_COUNT: u8 = 200;
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    });
}
pub fn handle_game_over(mut event_reader: EventReader<GameOverEvent>) {
    for event in event_reader.iter() {
        info!("Oyun sonlandı...Oyuncunun skoru {}", event.current_score);
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();
    let star_sizes = vec![0.25, 1., 1.25, 1.5, 1.75, 2.];
    for _ in 0..STARS_COUNT {
        let idx = rand::thread_rng().gen_range(0..star_sizes.len());

        let (x, y) = (
            rand::thread_rng().gen_range(0.0..window.width()),
            rand::thread_rng().gen_range(0.0..window.height()),
        );
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(star_sizes[idx]).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::ANTIQUE_WHITE)),
            transform: Transform::from_translation(Vec3::new(x, y, 0.)),
            ..default()
        });
    }
}