use crate::constants::{FLOOR_HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::shapes::pipe::Pipe;
use crate::shapes::pipe_bundle::PipeBundle;
use crate::shapes::pipe_type::PipeType;
use crate::shapes::square::{spawn_square, Square};
use crate::systems::animation::{AnimationIndices, AnimationTimer};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::{Collider, KinematicCharacterController, RigidBody};

pub fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("./textures/gabe-idle-run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24., 24.), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform {
                    scale: Vec3::splat(1.5),
                    translation: Vec3::new(
                        -SCREEN_WIDTH * 0.40,
                        -SCREEN_HEIGHT * 0.5 + FLOOR_HEIGHT * 2. + 5.,
                        0.,
                    ),
                    ..Default::default()
                },
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(1.5, 1.5))
        .insert(KinematicCharacterController::default());

    // commands
    //     .spawn(MaterialMesh2dBundle {
    //         mesh: meshes.add(shape::Circle::default().into()).into(),
    //         material: materials.add(ColorMaterial::from(Color::DARK_GREEN)),
    //         transform: Transform {
    //             translation: Vec3::new(-SCREEN_WIDTH * 0.5 + 30., 0., 0.),
    //             scale: Vec3::new(60., 60., 1.),
    //             ..Default::default()
    //         },
    //         ..default()
    //     })
    //     .insert(RigidBody::KinematicPositionBased)
    //     .insert(Collider::ball(0.5))
    //     .insert(KinematicCharacterController::default());

    let gold_pipe = Pipe {
        x: 0.,
        y: -SCREEN_HEIGHT * 0.5 + 50. + FLOOR_HEIGHT,
        z: 0.,
        hegiht: 100.,
        z_deep: 1.,
        color: Color::GOLD,
        pipe_type: PipeType::Big(60.),
    };
    commands.spawn(PipeBundle::new(gold_pipe));

    let white_pipe = Pipe {
        x: SCREEN_WIDTH * 0.1,
        y: -SCREEN_HEIGHT * 0.5 + 100. + FLOOR_HEIGHT,
        z: 0.,
        hegiht: 200.,
        z_deep: 1.,
        color: Color::ANTIQUE_WHITE,
        pipe_type: PipeType::Small(20.),
    };
    commands.spawn(PipeBundle::new(white_pipe));

    let purple_pipe = Pipe {
        x: SCREEN_WIDTH * 0.2,
        y: -SCREEN_HEIGHT * 0.5 + 50. + FLOOR_HEIGHT,
        z: 0.,
        hegiht: 100.,
        z_deep: 1.,
        color: Color::PURPLE,
        pipe_type: PipeType::Midi(40.),
    };
    commands.spawn(PipeBundle::new(purple_pipe));

    for _ in 0..3 {
        let square = Square(Color::ORANGE_RED);
        spawn_square(&mut commands, square);
    }

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::BISQUE)),
            transform: Transform {
                translation: Vec3::new(
                    -SCREEN_WIDTH * 0.25,
                    -SCREEN_HEIGHT * 0.5 + FLOOR_HEIGHT * 2. + 5.,
                    0.,
                ),
                scale: Vec3::new(25., 25., 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.5));

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
