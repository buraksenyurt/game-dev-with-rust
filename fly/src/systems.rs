use crate::components::*;
use crate::constants::*;
use crate::game_play::Level;
use crate::level_manager::*;
use bevy::prelude::*;
use bevy::sprite::Sprite;
use bevy_rapier2d::prelude::*;
use rand::prelude::IndexedRandom;
use rand::Rng;

pub fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let idle_texture: Handle<Image> = asset_server.load("Fox/Idle.png");
    let running_texture: Handle<Image> = asset_server.load("Fox/Run.png");
    let jumping_texture: Handle<Image> = asset_server.load("Fox/Jump.png");

    let frame_size = UVec2::new(32, 32);
    let idle_atlas = TextureAtlasLayout::from_grid(frame_size, 11, 1, None, None);
    let idle_atlas_handle = texture_atlases.add(idle_atlas);

    commands.spawn(Camera2d);

    // Player
    commands.spawn((
        Player,
        Sprite {
            image: idle_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: idle_atlas_handle,
                index: 0,
            }),
            custom_size: Some(Vec2::ONE * PLAYER_SIZE),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(PLAYER_SIZE / 2.0, PLAYER_SIZE / 2.0),
        GravityScale(GRAVITY_FORCE),
        Transform::from_xyz(PLAYER_START_X, 0.0, 0.0),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        Friction {
            coefficient: 0.2,
            combine_rule: CoefficientCombineRule::Min,
        },
        StandardAnimation::default(),
        PlayerState::Idle,
        PlayerAnimationHandles {
            idle: idle_texture,
            running: running_texture,
            jumping: jumping_texture,
        },
    ));

    load_level(Level::FirstGate, &mut commands, &asset_server);
}

fn load_level(level: Level, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let ground_tile_image: Handle<Image> = asset_server.load("Tile.png");
    let platform_image: Handle<Image> = asset_server.load("Platform.png");

    if let Some(platforms) = get_level_data(level) {
        for platform_data in platforms {
            let image = match platform_data.platform_type {
                PlatformType::Ground => ground_tile_image.clone(),
                PlatformType::Platform => platform_image.clone(),
                _ => continue,
            };

            spawn_platform(&platform_data, commands, &image);
        }
    }

    spawn_random_crates(8, commands, asset_server);
}

fn spawn_random_crates(count: usize, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let crate_image: Handle<Image> = asset_server.load("Crate.png");
    let mut rng = rand::rng();

    for i in 0..count {
        let location = Vec2::new(
            rng.random_range(-500.0..500.0),
            rng.random_range(400.0..500.0),
        );
        let crate_size = [32.0, 36.0, 40.0, 44.0].choose(&mut rng).unwrap();
        commands.spawn((
            Sprite {
                image: crate_image.clone(),
                custom_size: Some(Vec2::ONE * crate_size),
                ..default()
            },
            Transform::from_xyz(location.x + (i as f32 * crate_size) as f32, location.y, 0.0),
            GravityScale(BOX_GRAVITY_FORCE),
            RigidBody::Dynamic,
            Collider::cuboid(crate_size / 2.0, crate_size / 2.0),
            AdditionalMassProperties::Mass(50.0),
            Friction {
                coefficient: 3.5,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.2,
                combine_rule: CoefficientCombineRule::Average,
            },
            Damping {
                linear_damping: 1.0,
                angular_damping: 0.4,
            },
            Ccd::enabled(),
            Platform {
                platform_type: PlatformType::Crate,
            },
        ));
    }
}

fn spawn_platform(platform_data: &PlatformData, commands: &mut Commands, image: &Handle<Image>) {
    for i in 0..platform_data.tile_count {
        let p_data = platform_data.clone();
        let transform = match p_data.direction {
            PlatformDirection::Horizontal => Vec2::new(
                p_data.position.x + (i * TILE_SIZE as usize) as f32,
                p_data.position.y,
            )
            .extend(0.0),
            PlatformDirection::Vertical => Vec2::new(
                p_data.position.x,
                p_data.position.y + (i * TILE_SIZE as usize) as f32,
            )
            .extend(0.0),
        };
        commands.spawn((
            Sprite {
                image: image.clone(),
                custom_size: Some(Vec2::ONE * TILE_SIZE),
                ..default()
            },
            Transform::from_translation(transform),
            RigidBody::Fixed,
            Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
            Platform {
                platform_type: p_data.platform_type,
            },
        ));
    }
}

// fn draw_ground(location: Vec3, count: usize, commands: &mut Commands, image: &Handle<Image>) {
//     for i in 0..count {
//         commands.spawn((
//             Sprite {
//                 image: image.clone(),
//                 custom_size: Some(Vec2::ONE * TILE_SIZE),
//                 //anchor: Anchor::BottomCenter,
//                 ..default()
//             },
//             Transform::from_xyz(
//                 location.x + (i * TILE_SIZE as usize) as f32,
//                 location.y,
//                 location.z,
//             ),
//             RigidBody::Fixed,
//             Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
//             // Friction::coefficient(0.1),
//         ));
//     }
// }

pub fn player_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Sprite), With<Player>>,
) {
    for (mut velocity, mut sprite) in query.iter_mut() {
        if keyboard.pressed(KeyCode::ArrowRight) {
            velocity.linvel.x = PLAYER_SPEED;
            sprite.flip_x = false;
        } else if keyboard.pressed(KeyCode::ArrowLeft) {
            velocity.linvel.x = -PLAYER_SPEED;
            sprite.flip_x = true;
        } else {
            velocity.linvel.x = 0.0;
        }

        if keyboard.just_pressed(KeyCode::Space) {
            velocity.linvel.y = JUMP_FORCE;
        }
    }
}
pub fn apply_animation(
    time: Res<Time>,
    mut query: Query<(&mut StandardAnimation, &mut Sprite, &PlayerState)>,
) {
    for (mut animation, mut sprite, state) in query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                let max_frames = match state {
                    PlayerState::Idle | PlayerState::Running => 11, // Idle.png 11 kareden oluştuğu için
                    PlayerState::Jumping => 1, // Jump.png sadece 1 kareden oluşuyor
                };

                atlas.index = (atlas.index + 1) % max_frames;
            }
        }
    }
}

pub fn update_player_animation(
    mut query: Query<(&mut Sprite, &PlayerState, &PlayerAnimationHandles)>,
) {
    for (mut sprite, state, handles) in query.iter_mut() {
        match state {
            PlayerState::Idle => {
                sprite.image = handles.idle.clone();
            }
            PlayerState::Running => {
                sprite.image = handles.running.clone();
            }
            PlayerState::Jumping => {
                sprite.image = handles.jumping.clone();
            }
        }
    }
}

pub fn update_player_state(mut query: Query<(&mut PlayerState, &Velocity)>) {
    for (mut state, velocity) in query.iter_mut() {
        if velocity.linvel.y > 1.0 {
            *state = PlayerState::Jumping;
        } else if velocity.linvel.x.abs() > 0.1 {
            *state = PlayerState::Running;
        } else {
            *state = PlayerState::Idle;
        }
    }
}
