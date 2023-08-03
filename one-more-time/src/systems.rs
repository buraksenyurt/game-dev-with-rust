use crate::builder;
use crate::builder::get_file_name;
use crate::components::*;
use crate::constants::*;
use crate::enums::*;
use crate::resources::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use rand::Rng;

pub fn sys_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    builder::create_schene(&mut commands, &asset_server);

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

pub fn sys_player_movement(
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

pub fn sys_customer_movement(mut customers: Query<(&mut Transform, &Customer)>, time: Res<Time>) {
    for (mut transform, customer) in &mut customers {
        if transform.translation.x >= WINDOW_WIDTH / 9. && !customer.is_get {
            let velocity = customer.speed * time.delta_seconds();
            transform.translation.x -= velocity;
        }
    }
}

pub fn sys_leave_donut(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut donuts: Query<(Entity, &mut Donut)>,
    desks: Query<(Entity, &Desk)>,
    mut customers: Query<(Entity, &mut Customer)>,
) {
    if game_state.cook_donut_count == 0 {
        return;
    }
    if input.just_pressed(KeyCode::Space) {
        for (entity, mut donut) in &mut donuts {
            if donut.is_delivered {
                continue;
            }
            let current_location = donut.location;
            if current_location.x > 25. {
                for (_, desk) in &desks {
                    match desk.region {
                        Region::Upside => {
                            if current_location.y >= 32. {
                                info!("Donut üst bölgede");
                                if donut.donut_type == desk.donut_type.unwrap() {
                                    calc_sell_price(&mut game_state, &mut donut);

                                    customers
                                        .iter_mut()
                                        .filter(|(_, c)| c.donut_type == donut.donut_type)
                                        .for_each(|(_, mut c)| c.is_get = true);

                                    break;
                                } else {
                                    game_state.balance -= donut.penalty_cost;
                                }
                            }
                        }
                        Region::Center => {
                            if current_location.y >= -17.5 && current_location.y < 17.5 {
                                info!("Donut orta bölgede");
                                if donut.donut_type == desk.donut_type.unwrap() {
                                    calc_sell_price(&mut game_state, &mut donut);
                                    customers
                                        .iter_mut()
                                        .filter(|(_, c)| c.donut_type == donut.donut_type)
                                        .for_each(|(_, mut c)| c.is_get = true);
                                } else {
                                    game_state.balance -= donut.penalty_cost;
                                }
                            }
                        }
                        Region::Downside => {
                            if current_location.y <= -32. {
                                info!("Donut alt bölgede");
                                if donut.donut_type == desk.donut_type.unwrap() {
                                    calc_sell_price(&mut game_state, &mut donut);
                                    customers
                                        .iter_mut()
                                        .filter(|(_, c)| c.donut_type == donut.donut_type)
                                        .for_each(|(_, mut c)| c.is_get = true);
                                } else {
                                    game_state.balance -= donut.penalty_cost;
                                }
                            }
                        }
                    }
                }
            }
            commands.entity(entity).despawn();
            game_state.cook_donut_count -= 1;
        }
    }
}

fn calc_sell_price(game_state: &mut ResMut<GameState>, donut: &mut Mut<Donut>) {
    info!("Çörek türleri de aynı. '{}'", donut.donut_type);
    donut.is_delivered = true;
    let price = match donut.donut_type {
        DonutType::Blue => 25.,
        DonutType::White => 50.,
        DonutType::Red => 125.,
    };
    info!("Sell price {}", price * 1.15);
    game_state.balance += price * 1.15;
}

pub fn sys_spawn_donut(
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
    let player_transform = *player.single();
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

        let target_location = Transform::from_xyz(
            player_transform.translation.x + DONUT_DISTANCE_FROM_COOK,
            player_transform.translation.y,
            player_transform.translation.z,
        );

        let donut = Donut {
            life_time: Timer::from_seconds(DEFAULT_DONUT_LIFE_TIME, TimerMode::Once),
            donut_type,
            is_delivered: false,
            is_leaved: false,
            location: target_location.translation,
            penalty_cost: match donut_type {
                DonutType::Blue => 12.5,
                DonutType::White => 17.5,
                DonutType::Red => 8.5,
            },
        };

        commands.spawn((
            SpriteBundle {
                texture,
                transform: target_location,
                ..default()
            },
            donut,
        ));
        game_state.cook_donut_count += 1;
    }
}

pub fn sys_donut_movement(
    mut donuts: Query<(&mut Transform, &mut Donut)>,
    player: Query<(&Transform, &Player), Without<Donut>>,
) {
    for (mut transform, mut donut) in &mut donuts {
        let player_transform = player.single();
        transform.translation.x = player_transform.0.translation.x + DONUT_DISTANCE_FROM_COOK;
        transform.translation.y = player_transform.0.translation.y;
        transform.translation.z = player_transform.0.translation.z;
        donut.location = transform.translation;
    }
}

pub fn sys_claim_donut(
    mut commands: Commands,
    time: Res<Time>,
    mut donuts: Query<(Entity, &mut Donut)>,
    mut game_state: ResMut<GameState>,
) {
    for (entity, mut donut) in &mut donuts {
        donut.life_time.tick(time.delta());
        if donut.life_time.finished() {
            game_state.cook_donut_count -= 1;
            commands.entity(entity).despawn();
        }
    }
}

pub fn sys_show_scoreboard(
    mut query: Query<&mut Text, With<ScoreText>>,
    game_state: ResMut<GameState>,
) {
    for mut text in &mut query {
        text.sections[1].value = format!("{}", game_state.balance);
    }
}

pub fn sys_return_customers(
    mut commands: Commands,
    mut customers: Query<(&mut Transform, &mut Customer, Entity)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for (mut transform, mut customer, e) in customers.iter_mut() {
        if customer.is_get {
            let velocity = customer.speed * time.delta_seconds();
            transform.translation.x += velocity;
            if transform.translation.x >= WINDOW_WIDTH * 0.5 - transform.translation.x {
                customer.is_get = false;
                commands.entity(e).despawn();
                info!("{:?} {:?} despawn işlemi", e, transform.translation);
                spawn_new_customer(&mut commands, &asset_server, customer);
            }
        }
    }
}

fn spawn_new_customer(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    customer: Mut<Customer>,
) {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(0..=2);
    let donuts = vec![DonutType::Blue, DonutType::White, DonutType::Red];
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(get_file_name(donuts[number])),
            transform: Transform::from_xyz(
                200.,
                match customer.direction {
                    Region::Upside => 50.,
                    Region::Center => 0.,
                    Region::Downside => -50.,
                },
                0.,
            ),
            ..default()
        },
        Customer {
            speed: 65.,
            donut_type: donuts[number],
            is_get: false,
            direction: customer.direction,
        },
    ));
}
