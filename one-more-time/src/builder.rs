use crate::components::*;
use crate::constants::CUSTOMER_WAIT_TIME;
use crate::enums::*;
use bevy::prelude::*;
use rand::Rng;

pub fn create_schene(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let mut directions: Vec<Region> = Vec::new();
    let mut rng = rand::thread_rng();

    loop {
        if directions.len() == 3 {
            break;
        }
        let number = rng.gen_range(0..=2);
        match number {
            0 => {
                if !directions.contains(&Region::Upside) {
                    directions.push(Region::Upside)
                }
            }
            1 => {
                if !directions.contains(&Region::Downside) {
                    directions.push(Region::Downside)
                }
            }
            _ => {
                if !directions.contains(&Region::Center) {
                    directions.push(Region::Center)
                }
            }
        }
    }

    let donuts = vec![DonutType::Blue, DonutType::White, DonutType::Red];
    for i in 0..=2 {
        let y = match directions[i] {
            Region::Upside => 50.,
            Region::Center => 0.,
            Region::Downside => -50.,
        };
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(get_file_name(donuts[i])),
                transform: Transform::from_xyz(200., y, 0.),
                ..default()
            },
            Customer {
                speed: 65.,
                donut_type: donuts[i],
                is_get: false,
                region: directions[i],
                life_time: Timer::from_seconds(CUSTOMER_WAIT_TIME, TimerMode::Once),
                can_return: false,
            },
        ));
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("desk.png"),
                transform: Transform::from_xyz(50., y, 0.),
                ..default()
            },
            Desk {
                region: directions[i],
                donut_type: Some(donuts[i]),
            },
        ));
    }
}

pub fn get_file_name(donut_type: DonutType) -> String {
    match donut_type {
        DonutType::Blue => "customer_blue.png".to_string(),
        DonutType::White => "customer_white.png".to_string(),
        DonutType::Red => "customer_red.png".to_string(),
    }
}
