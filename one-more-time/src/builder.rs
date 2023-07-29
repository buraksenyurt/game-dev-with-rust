use crate::components::*;
use crate::enums::*;
use bevy::prelude::*;
use rand::Rng;

pub fn create_desks(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let desk_texture = asset_server.load("desk.png");
    commands.spawn((SpriteBundle {
        texture: desk_texture,
        transform: Transform::from_xyz(50., 50., 0.),
        ..default()
    },));
    let desk_texture = asset_server.load("desk.png");
    commands.spawn((SpriteBundle {
        texture: desk_texture,
        transform: Transform::from_xyz(50., 0., 0.),
        ..default()
    },));
    let desk_texture = asset_server.load("desk.png");
    commands.spawn((SpriteBundle {
        texture: desk_texture,
        transform: Transform::from_xyz(50., -50., 0.),
        ..default()
    },));
}

pub fn create_customers(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let mut directions: Vec<CustomerDirection> = Vec::new();
    let mut rng = rand::thread_rng();

    loop {
        if directions.len() == 3 {
            break;
        }
        let number = rng.gen_range(0..=2);
        match number {
            0 => {
                if !directions.contains(&CustomerDirection::Up) {
                    directions.push(CustomerDirection::Up)
                }
            }
            1 => {
                if !directions.contains(&CustomerDirection::Down) {
                    directions.push(CustomerDirection::Down)
                }
            }
            _ => {
                if !directions.contains(&CustomerDirection::Center) {
                    directions.push(CustomerDirection::Center)
                }
            }
        }
    }

    let customer_texture = asset_server.load("customer_blue.png");
    let y = match directions[0] {
        CustomerDirection::Up => 50.,
        CustomerDirection::Center => 0.,
        CustomerDirection::Down => -50.,
    };
    commands.spawn((
        SpriteBundle {
            texture: customer_texture,
            transform: Transform::from_xyz(200., y, 0.),
            ..default()
        },
        Customer { speed: 65. },
    ));
    let customer_texture = asset_server.load("customer_white.png");
    let y = match directions[1] {
        CustomerDirection::Up => 50.,
        CustomerDirection::Center => 0.,
        CustomerDirection::Down => -50.,
    };
    commands.spawn((
        SpriteBundle {
            texture: customer_texture,
            transform: Transform::from_xyz(200., y, 0.),
            ..default()
        },
        Customer { speed: 45. },
    ));
    let customer_texture = asset_server.load("customer_red.png");
    let y = match directions[2] {
        CustomerDirection::Up => 50.,
        CustomerDirection::Center => 0.,
        CustomerDirection::Down => -50.,
    };
    commands.spawn((
        SpriteBundle {
            texture: customer_texture,
            transform: Transform::from_xyz(200., y, 0.),
            ..default()
        },
        Customer { speed: 35. },
    ));
}
