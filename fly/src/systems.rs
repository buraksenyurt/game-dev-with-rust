use crate::components::*;
use crate::constants::*;
use crate::game_play::Level;
use bevy::prelude::*;
use bevy::sprite::Sprite;
use bevy_rapier2d::prelude::*;

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
            custom_size: Some(Vec2::ONE * 32.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(16.0, 16.0),
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

    load_level_tiles(Level::FirstGate, &mut commands, &asset_server);
}

fn load_level_tiles(_game_level: Level, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let ground_tile_image: Handle<Image> = asset_server.load("Tile.png");
    let platform_image: Handle<Image> = asset_server.load("Platform.png");

    // Main Ground
    draw_ground(
        Vec3::new(-600.0, GROUND_LEVEL, 0.0),
        36,
        commands,
        &ground_tile_image,
    );

    // Jumping grounds
    draw_ground(Vec3::new(-600.0, 100.0, 0.0), 3, commands, &platform_image);
    draw_ground(Vec3::new(-500.0, 180.0, 0.0), 4, commands, &platform_image);
    draw_ground(Vec3::new(-360.0, 140.0, 0.0), 2, commands, &platform_image);
    draw_ground(Vec3::new(-280.0, 60.0, 0.0), 4, commands, &platform_image);
    draw_ground(Vec3::new(-100.0, 80.0, 0.0), 3, commands, &platform_image);
    draw_ground(Vec3::new(20.0, 150.0, 0.0), 5, commands, &platform_image);
    draw_ground(Vec3::new(160.0, 60.0, 0.0), 3, commands, &platform_image);
}

fn draw_ground(location: Vec3, count: usize, commands: &mut Commands, image: &Handle<Image>) {
    for i in 0..count {
        commands.spawn((
            Sprite {
                image: image.clone(),
                custom_size: Some(Vec2::ONE * 24.0),
                //anchor: Anchor::BottomCenter,
                ..default()
            },
            Transform::from_xyz(location.x + (i * 24) as f32, location.y, location.z),
            RigidBody::Fixed,
            Collider::cuboid(12.0, 12.0),
            // Friction::coefficient(0.1),
        ));
    }
}

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
