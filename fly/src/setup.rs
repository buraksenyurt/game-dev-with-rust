use crate::constants::{GROUND_LEVEL, PLAYER_X};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::prelude::{
    AssetServer, Camera2d, Commands, Component, Handle, Image, Res, ResMut, Transform, Vec2, Vec3,
};
use bevy::sprite::{Anchor, Sprite};
use bevy::utils::default;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(Vec3);

pub fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_image: Handle<Image> = asset_server.load("Fox.png");
    let tile_image: Handle<Image> = asset_server.load("Tile.png");

    commands.spawn(Camera2d::default());

    // Player
    commands.spawn((
        Player,
        Sprite {
            image: player_image,
            custom_size: Some(Vec2::ONE * 48.0),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
        Velocity(Vec3::ZERO),
    ));

    // Ground
    for i in 0..36 {
        commands.spawn((
            Sprite {
                image: tile_image.clone(),
                custom_size: Some(Vec2::ONE * 32.0),
                anchor: Anchor::TopLeft,
                ..default()
            },
            Transform::from_xyz(-600.0 + (i * 32) as f32, GROUND_LEVEL, 0.0),
        ));
    }
}
