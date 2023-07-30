mod builder;
mod components;
mod enums;
mod resources;

use crate::components::*;
use crate::enums::*;
use crate::resources::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::DefaultPlugins;
use rand::Rng;

const MOVEMENT_SPEED: f32 = 120.;
const BALANCE_INIT_VALUE: f32 = 1000.;
const DEFAULT_DONUT_LIFE_TIME: f32 = 10.;
const DONUT_COST: f32 = 100.;
const WINDOW_WIDTH: f32 = 640.;
const WINDOW_HEIGHT: f32 = 480.;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "One More Time Blocky".into(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(GameState {
            balance: BALANCE_INIT_VALUE,
            cook_donut_count: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement,
                spawn_donut,
                claim_donut,
                scoreboard,
                customer_movement,
                donut_movement,
                leave_donut,
            ),
        )
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

    builder::create_desks(&mut commands, &asset_server);
    builder::create_customers(&mut commands, &asset_server);

    let hero_texture = asset_server.load("blocky.png");
    commands.spawn((
        SpriteBundle {
            texture: hero_texture,
            ..default()
        },
        Player {
            speed: MOVEMENT_SPEED,
        },
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Kasa: ",
                TextStyle {
                    font_size: 24.0,
                    color: Color::GOLD,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 24.0,
                color: Color::GOLD,
                ..default()
            }),
        ]),
        ScoreText,
    ));
}

fn player_movement(
    mut player: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut player {
        let velocity = player.speed * time.delta_seconds();
        if input.pressed(KeyCode::W)
            || input.pressed(KeyCode::Up) && transform.translation.y <= WINDOW_HEIGHT / 6.5
        {
            transform.translation.y += velocity;
        }
        if input.pressed(KeyCode::S)
            || input.pressed(KeyCode::Down) && transform.translation.y >= -WINDOW_HEIGHT / 6.
        {
            transform.translation.y -= velocity;
        }
        if input.pressed(KeyCode::D)
            || input.pressed(KeyCode::Right) && transform.translation.x <= WINDOW_WIDTH / 22.
        {
            transform.translation.x += velocity;
        }
        if input.pressed(KeyCode::A)
            || input.pressed(KeyCode::Left) && transform.translation.x >= -WINDOW_WIDTH / 6.
        {
            transform.translation.x -= velocity;
        }
    }
}

fn customer_movement(mut customers: Query<(&mut Transform, &Customer)>, time: Res<Time>) {
    for (mut transform, customer) in &mut customers {
        if transform.translation.x >= WINDOW_WIDTH / 9. {
            let velocity = customer.speed * time.delta_seconds();
            transform.translation.x -= velocity;
        }
    }
}

fn leave_donut(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut donuts: Query<(Entity, &Donut)>,
) {
    if game_state.cook_donut_count == 0 {
        return;
    }
    if input.just_pressed(KeyCode::Space) {
        for (entity, donut) in &mut donuts {
            commands.entity(entity).despawn();
            game_state.cook_donut_count -= 1;
            if !donut.is_delivered {
                game_state.balance -= 10.;
            }
        }
    }
}
fn spawn_donut(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    if game_state.cook_donut_count == 1 {
        return;
    }
    let player_transform = player.single().clone();
    if game_state.balance >= DONUT_COST {
        game_state.balance -= DONUT_COST;

        info!("Oyuncunun kalan altını {}", game_state.balance);
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(1..10);
        let (texture, donut_type) = match number {
            1 | 3 | 5 => (asset_server.load("blue_donut.png"), DonutType::Blue),
            2 | 4 | 6 => (asset_server.load("red_donut.png"), DonutType::Red),
            _ => (asset_server.load("white_donut.png"), DonutType::White),
        };

        let donut = Donut {
            life_time: Timer::from_seconds(DEFAULT_DONUT_LIFE_TIME, TimerMode::Once),
            donut_type,
            is_delivered: false,
            is_leaved: false,
        };

        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_xyz(
                    player_transform.translation.x + 15.,
                    player_transform.translation.y,
                    player_transform.translation.z,
                ),
                ..default()
            },
            donut,
        ));
        game_state.cook_donut_count += 1;
    }
}

fn donut_movement(
    mut donuts: Query<(&mut Transform, &Donut)>,
    player: Query<(&Transform, &Player), Without<Donut>>,
) {
    for (mut transform, _donut) in &mut donuts {
        let player_transform = player.single();
        transform.translation.x = player_transform.0.translation.x + 10.;
        transform.translation.y = player_transform.0.translation.y;
        transform.translation.z = player_transform.0.translation.z;
    }
}
fn claim_donut(
    mut commands: Commands,
    time: Res<Time>,
    mut donuts: Query<(Entity, &mut Donut)>,
    mut game_state: ResMut<GameState>,
) {
    for (entity, mut donut) in &mut donuts {
        donut.life_time.tick(time.delta());
        if donut.life_time.finished() {
            //info!("{:?} ID li entity yok ediliyor", entity);
            let price = match donut.donut_type {
                DonutType::Blue => 25.,
                DonutType::White => 50.,
                DonutType::Red => 125.,
            };
            if donut.is_delivered {
                game_state.balance += price * 1.15;
            }
            game_state.cook_donut_count -= 1;
            commands.entity(entity).despawn();
            info!(
                "Donut {} altına satıldı. Güncel altın miktarı {}",
                price, game_state.balance
            );
        }
    }
}

fn scoreboard(mut query: Query<&mut Text, With<ScoreText>>, game_state: ResMut<GameState>) {
    for mut text in &mut query {
        text.sections[1].value = format!("{}", game_state.balance);
    }
}
