use crate::components::*;
use crate::enums::*;
use bevy::prelude::*;
use rand::Rng;

pub fn create_desks(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let desk_texture = asset_server.load("desk.png");
    commands.spawn((
        SpriteBundle {
            texture: desk_texture,
            transform: Transform::from_xyz(50., 50., 0.),
            ..default()
        },
        Desk {
            region: Region::Upside,
            donut_type: None,
        },
    ));
    let desk_texture = asset_server.load("desk.png");
    commands.spawn((
        SpriteBundle {
            texture: desk_texture,
            transform: Transform::from_xyz(50., 0., 0.),
            ..default()
        },
        Desk {
            region: Region::Center,
            donut_type: None,
        },
    ));
    let desk_texture = asset_server.load("desk.png");
    commands.spawn((
        SpriteBundle {
            texture: desk_texture,
            transform: Transform::from_xyz(50., -50., 0.),
            ..default()
        },
        Desk {
            region: Region::Downside,
            donut_type: None,
        },
    ));
}

pub fn create_customers(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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

    let customer_texture = asset_server.load("customer_blue.png");
    let y = match directions[0] {
        Region::Upside => 50.,
        Region::Center => 0.,
        Region::Downside => -50.,
    };
    commands.spawn((
        SpriteBundle {
            texture: customer_texture,
            transform: Transform::from_xyz(200., y, 0.),
            ..default()
        },
        Customer {
            speed: 65.,
            donut_type: DonutType::Blue,
        },
    ));
    let customer_texture = asset_server.load("customer_white.png");
    let y = match directions[1] {
        Region::Upside => 50.,
        Region::Center => 0.,
        Region::Downside => -50.,
    };
    commands.spawn((
        SpriteBundle {
            texture: customer_texture,
            transform: Transform::from_xyz(200., y, 0.),
            ..default()
        },
        Customer {
            speed: 45.,
            donut_type: DonutType::White,
        },
    ));
    let customer_texture = asset_server.load("customer_red.png");
    let y = match directions[2] {
        Region::Upside => 50.,
        Region::Center => 0.,
        Region::Downside => -50.,
    };
    commands.spawn((
        SpriteBundle {
            texture: customer_texture,
            transform: Transform::from_xyz(200., y, 0.),
            ..default()
        },
        Customer {
            speed: 35.,
            donut_type: DonutType::Red,
        },
    ));
}
