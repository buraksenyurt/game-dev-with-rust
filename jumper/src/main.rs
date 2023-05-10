mod constants;
mod pipe;

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH, STD_PIPE_WIDTH};
use crate::pipe::{spawn_pipe, Pipe};
use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResolution;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::NAVY))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Jumper - A Series Jumping Game :P".to_string(),
                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::default().into()).into(),
        material: materials.add(ColorMaterial::from(Color::DARK_GREEN)),
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::new(60., 60., 1.),
            ..Default::default()
        },
        ..default()
    });

    let gold_pipe = Pipe {
        x: 0.,
        y: -SCREEN_HEIGHT * 0.5 + 50.,
        z: 0.,
        width: STD_PIPE_WIDTH,
        hegiht: 100.,
        z_deep: 1.,
        color: Color::GOLD,
    };
    spawn_pipe(&mut commands, gold_pipe);

    let white_pipe = Pipe {
        x: SCREEN_WIDTH * 0.1,
        y: -SCREEN_HEIGHT * 0.5 + 100.,
        z: 0.,
        width: STD_PIPE_WIDTH,
        hegiht: 200.,
        z_deep: 1.,
        color: Color::ANTIQUE_WHITE,
    };
    spawn_pipe(&mut commands, white_pipe);

    commands.spawn(Camera2dBundle::default());
}
