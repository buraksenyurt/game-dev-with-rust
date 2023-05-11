use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH, STD_PIPE_WIDTH};
use crate::shapes::pipe::{spawn_pipe, Pipe};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::{Collider, RigidBody};

pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::DARK_GREEN)),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(60., 60., 1.),
                ..Default::default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.5));

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
