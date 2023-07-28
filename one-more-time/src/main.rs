use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::DefaultPlugins;
use rand::Rng;

const MOVEMENT_SPEED: f32 = 120.;
const DEFAULT_GOLD_VALUE: i32 = 1000;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "One More Time Blocky".into(),
                        resolution: (640., 480.).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(Gold(DEFAULT_GOLD_VALUE))
        .add_systems(Startup, setup)
        .add_systems(Update, (movement, spawn_donut))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb_u8(0, 143, 17)),
        },
        ..default()
    };
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.,
        min_height: 144.,
    };
    commands.spawn(camera);
    let hero_texture = asset_server.load("blocky.png");
    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         custom_size: Some(Vec2::new(33., 48.)),
    //         ..default()
    //     },
    //     texture: hero_texture,
    //     ..default()
    // });
    commands.spawn((
        SpriteBundle {
            texture: hero_texture,
            ..default()
        },
        Player {
            speed: MOVEMENT_SPEED,
        },
    ));
}

fn movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let velocity = player.speed * time.delta_seconds();
        if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
            transform.translation.y += velocity;
        }
        if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
            transform.translation.y -= velocity;
        }
        if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
            transform.translation.x += velocity;
        }
        if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
            transform.translation.x -= velocity;
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Resource)]
pub struct Gold(pub i32);

fn spawn_donut(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut gold: ResMut<Gold>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    let player_transform = player.single();
    if gold.0 >= 100 {
        gold.0 -= 100;

        info!("Oyuncunun kalan altını {}", gold.0);
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(1..10);
        let texture = match number {
            1 | 3 | 5 => asset_server.load("blue_donut.png"),
            2 | 4 | 6 => asset_server.load("red_donut.png"),
            _ => asset_server.load("white_donut.png"),
        };

        commands.spawn(SpriteBundle {
            texture,
            transform: *player_transform,
            ..default()
        });
    }
}
