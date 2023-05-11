use crate::constants::{FLOOR_HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::shapes::pipe::{spawn_pipe, Pipe};
use crate::shapes::pipe_type::PipeType;
use crate::shapes::square::{spawn_square, Square};
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
        hegiht: 100.,
        z_deep: 1.,
        color: Color::GOLD,
        pipe_type: PipeType::BIG,
    };
    spawn_pipe(&mut commands, gold_pipe);

    let white_pipe = Pipe {
        x: SCREEN_WIDTH * 0.1,
        y: -SCREEN_HEIGHT * 0.5 + 100.,
        z: 0.,
        hegiht: 200.,
        z_deep: 1.,
        color: Color::ANTIQUE_WHITE,
        pipe_type: PipeType::SMALL,
    };
    spawn_pipe(&mut commands, white_pipe);

    let purple_pipe = Pipe {
        x: SCREEN_WIDTH * 0.2,
        y: -SCREEN_HEIGHT * 0.5 + 50.,
        z: 0.,
        hegiht: 100.,
        z_deep: 1.,
        color: Color::PURPLE,
        pipe_type: PipeType::MIDI,
    };
    spawn_pipe(&mut commands, purple_pipe);

    for _ in 0..3 {
        let square = Square(Color::ORANGE_RED);
        spawn_square(&mut commands, square);
    }

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::AZURE,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., -SCREEN_HEIGHT * 0.5 + FLOOR_HEIGHT * 0.5, 0.),
                scale: Vec3::new(SCREEN_WIDTH, FLOOR_HEIGHT, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5));

    commands.spawn(Camera2dBundle::default());
}
