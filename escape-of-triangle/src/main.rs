mod constants;

use crate::constants::*;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Direction(Vec2);

#[derive(Component)]
struct Tower;

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct TowerBundle {
    tower: Tower,
    position: Position,
}

impl TowerBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            tower: Tower,
            position: Position(Vec2::new(x, y)),
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    direction: Direction,
    position: Position,
    velocity: Velocity,
}

impl PlayerBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            player: Player,
            direction: Direction(Vec2::ZERO),
            position: Position(Vec2::new(x, y)),
            velocity: Velocity(Vec2::ZERO),
        }
    }
}
fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let w_height = window.resolution.height();
        let w_width = window.resolution.width();

        let shape = Mesh::from(RegularPolygon::new(TRIANGLE_CIRCUS_RADIUS, 3));
        let color = ColorMaterial::from(TRIANGLE_COLOR);

        let mesh_handle = meshes.add(shape);
        let material_handle = materials.add(color);

        commands.spawn((
            PlayerBundle::new(w_width * 0.25, w_height * 0.25),
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                ..default()
            },
        ));
    }
}

fn spawn_towers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let w_height = window.resolution.height();
        let w_width = window.resolution.width();
        let positions = [
            Vec2::new(-w_width / 2. + TOWER_WIDTH, w_height / 2. - TOWER_HEIGHT), // Upper Left Cornet
            Vec2::new(w_width / 2. - TOWER_WIDTH, w_height / 2. - TOWER_HEIGHT), // Upper Right Corner
            Vec2::new(0., 0.),                                                   // Center
            Vec2::new(-w_width / 2. + TOWER_WIDTH, -w_height / 2. + TOWER_HEIGHT), // Down Left Cornet
            Vec2::new(w_width / 2. - TOWER_WIDTH, -w_height / 2. + TOWER_HEIGHT), // Down Right Corner
        ];

        for position in positions.iter() {
            let shape = Mesh::from(Rectangle::new(TOWER_WIDTH, TOWER_HEIGHT));
            let color = ColorMaterial::from(TOWER_COLOR);

            let mesh_handle = meshes.add(shape);
            let material_handle = materials.add(color);

            commands.spawn((
                TowerBundle::new(position.x, position.y),
                MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: material_handle,
                    ..default()
                },
            ));
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_towers, spawn_camera, spawn_player))
        .add_systems(Update, (draw_towers, draw_player))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}

fn draw_towers(mut positions: Query<(&mut Transform, &Position), With<Tower>>) {
    for (mut transform, position) in &mut positions {
        transform.translation = position.0.extend(0.);
    }
}

fn draw_player(mut positions: Query<(&mut Transform, &Position), With<Player>>) {
    for (mut transform, position) in &mut positions {
        transform.translation = position.0.extend(0.);
    }
}
