use crate::common::{get_donut_cost, get_donut_penalty_cost};
use crate::components::*;
use crate::constants::*;
use crate::enums::*;
use crate::resources::*;
use crate::{builder, common};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use rand::Rng;
use std::process::exit;

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
    mut game_state: ResMut<LiveParameters>,
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
            let donut_location = donut.location;
            if donut_location.x > 25. {
                for (_, desk) in &desks {
                    match desk.region {
                        Region::Upside => {
                            if donut_location.y >= 32. {
                                info!("Donut üst bölgede. Desk Type {}", desk.donut_type.unwrap());
                                if donut.donut_type == desk.donut_type.unwrap() {
                                    calc_sell_price(&mut game_state, &mut donut);

                                    customers
                                        .iter_mut()
                                        .filter(|(_, c)| {
                                            c.donut_type == donut.donut_type
                                                && c.region == Region::Upside
                                        })
                                        .for_each(|(_, mut c)| c.is_get = true);

                                    break;
                                } else {
                                    game_state.balance -= get_donut_penalty_cost(donut.donut_type);
                                }
                            }
                        }
                        Region::Center => {
                            if donut_location.y >= -17.5 && donut_location.y < 17.5 {
                                info!("Donut orta bölgede. Desk Type {}", desk.donut_type.unwrap());
                                if donut.donut_type == desk.donut_type.unwrap() {
                                    calc_sell_price(&mut game_state, &mut donut);
                                    customers
                                        .iter_mut()
                                        .filter(|(_, c)| {
                                            c.donut_type == donut.donut_type
                                                && c.region == Region::Center
                                        })
                                        .for_each(|(_, mut c)| c.is_get = true);
                                } else {
                                    game_state.balance -= get_donut_penalty_cost(donut.donut_type);
                                }
                            }
                        }
                        Region::Downside => {
                            if donut_location.y <= -32. {
                                info!("Donut alt bölgede. Desk Type {}", desk.donut_type.unwrap());
                                if donut.donut_type == desk.donut_type.unwrap() {
                                    calc_sell_price(&mut game_state, &mut donut);
                                    customers
                                        .iter_mut()
                                        .filter(|(_, c)| {
                                            c.donut_type == donut.donut_type
                                                && c.region == Region::Downside
                                        })
                                        .for_each(|(_, mut c)| c.is_get = true);
                                } else {
                                    game_state.balance -= get_donut_penalty_cost(donut.donut_type);
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

fn calc_sell_price(game_state: &mut ResMut<LiveParameters>, donut: &mut Mut<Donut>) {
    info!("Çörek türleri de aynı. '{}'", donut.donut_type);
    donut.is_delivered = true;
    let price = get_donut_cost(donut.donut_type);
    info!("Sell price {}", price * 1.15);
    game_state.balance += price * 1.15;
}

pub fn sys_spawn_donut(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut game_state: ResMut<LiveParameters>,
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
        info!("Oyuncunun kalan altını {}", game_state.balance);
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(1..10);
        let (texture, donut_type) = match number {
            1 | 3 | 5 => (asset_server.load("blue_donut.png"), DonutType::Blue),
            2 | 4 | 6 => (asset_server.load("red_donut.png"), DonutType::Red),
            _ => (asset_server.load("white_donut.png"), DonutType::White),
        };
        game_state.balance -= get_donut_cost(donut_type);
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
    mut game_state: ResMut<LiveParameters>,
) {
    for (entity, mut donut) in &mut donuts {
        donut.life_time.tick(time.delta());
        if donut.life_time.finished() {
            game_state.cook_donut_count -= 1;
            commands.entity(entity).despawn();
        }
    }
}

pub fn sys_check_waiting_customers(time: Res<Time>, mut customers: Query<(&mut Customer, Entity)>) {
    for (mut cust, _entity) in customers.iter_mut() {
        if !cust.can_return {
            cust.life_time.tick(time.delta());
            if cust.life_time.finished() && !cust.is_get {
                info!("Customer can return");
                cust.can_return = true;
            }
        }
    }
}

pub fn sys_claim_waiting_customers(
    mut commands: Commands,
    mut customers: Query<(&mut Transform, &Customer, Entity)>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<LiveParameters>,
    _time: Res<Time>,
) {
    for (mut transform, customer, entity) in customers.iter_mut() {
        if customer.can_return {
            transform.translation.x += 1.75; // * time.delta_seconds(); delta_seconds koyunca olduğu yerde sayıyor
            if transform.translation.x >= WINDOW_WIDTH * 0.5 - transform.translation.x {
                info!("Customer despawned on x={}", transform.translation.x);
                commands.entity(entity).despawn();
                let donut_type = get_random_donut();
                spawn_new_customer(&mut commands, &asset_server, donut_type, customer.region);

                game_state.balance -= get_donut_penalty_cost(donut_type);
            }
        }
    }
}

pub fn sys_show_scoreboard(
    mut query: Query<&mut Text, With<ScoreText>>,
    game_state: ResMut<LiveParameters>,
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
    mut desks: Query<(&mut Desk, Entity)>,
) {
    for (mut transform, mut customer, e) in customers.iter_mut() {
        if customer.is_get {
            let velocity = customer.speed * time.delta_seconds();
            transform.translation.x += velocity;
            if transform.translation.x >= WINDOW_WIDTH * 0.5 - transform.translation.x {
                customer.is_get = false;
                commands.entity(e).despawn();
                info!("{:?} {:?} despawn işlemi", e, transform.translation);
                let new_donut_type = get_random_donut();
                spawn_new_customer(
                    &mut commands,
                    &asset_server,
                    new_donut_type,
                    customer.region,
                );
                for (mut desk, _entity) in desks.iter_mut() {
                    if desk.region == customer.region {
                        desk.donut_type = Some(new_donut_type);
                        break;
                    }
                }
            }
        }
    }
}

fn get_random_donut() -> DonutType {
    let mut rng = rand::thread_rng();
    let donuts = vec![DonutType::Blue, DonutType::White, DonutType::Red];
    let number = rng.gen_range(0..=2);
    donuts[number]
}

fn spawn_new_customer(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    new_donut_type: DonutType,
    region: Region,
) {
    let customer = Customer {
        speed: 65.,
        donut_type: new_donut_type,
        is_get: false,
        region,
        life_time: Timer::from_seconds(CUSTOMER_WAIT_TIME, TimerMode::Once),
        can_return: false,
    };
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(common::get_file_name(new_donut_type)),
            transform: Transform::from_xyz(
                200.,
                match region {
                    Region::Upside => 50.,
                    Region::Center => 0.,
                    Region::Downside => -50.,
                },
                0.,
            ),
            ..default()
        },
        customer,
    ));
}

pub fn sys_start_game(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::G) {
        info!("Oyun başlatılıyor");
        next_state.set(GameState::Playing);
    } else if keys.just_pressed(KeyCode::X) {
        info!("Oyundan çıkılıyor");
        exit(1);
    }
}

pub fn sys_show_menu(mut commands: Commands) {
    let mut main_menu = commands.spawn((
        Name::new("MainMenu"),
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
    ));

    main_menu.with_children(|m| {
        let mut canvas = m.spawn((
            Name::new("canvas"),
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ));

        canvas.with_children(|c| {
            c.spawn(NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|menu| {
                menu.spawn((
                    Menu,
                    TextBundle::from_section(
                        "Press G for start",
                        TextStyle {
                            font_size: MENU_TEXT_SIZE,
                            color: Color::GOLD,
                            ..default()
                        },
                    )
                    .with_text_alignment(TextAlignment::Center)
                    .with_background_color(Color::BLACK),
                ));
            })
            .with_children(|menu| {
                menu.spawn((
                    Menu,
                    TextBundle::from_section(
                        "X for Exit",
                        TextStyle {
                            font_size: MENU_TEXT_SIZE,
                            color: Color::GOLD,
                            ..default()
                        },
                    )
                    .with_text_alignment(TextAlignment::Center)
                    .with_background_color(Color::BLACK),
                ));
            });
        });
    });
}

pub fn sys_change_visibility(
    mut menu: Query<&mut Visibility, With<Menu>>,
    game_state: Res<State<GameState>>,
) {
    if !game_state.is_changed() {
        return;
    }
    for mut m in &mut menu {
        let game_state = game_state.get();
        if *game_state == GameState::Playing {
            *m = Visibility::Hidden;
        } else {
            *m = Visibility::Visible;
        }
    }
}
