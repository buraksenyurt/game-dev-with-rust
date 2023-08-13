use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use bevy::prelude::{Color, Commands, Sprite, SpriteBundle, Transform, Vec3};
use bevy_rapier2d::prelude::{Collider, RigidBody};
use rand::Rng;

pub struct Square(pub Color);

pub fn spawn_square(commands: &mut Commands, s: Square) {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(0.0..SCREEN_WIDTH * 0.5);
    let y = rng.gen_range(0.0..SCREEN_HEIGHT * 0.5);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: s.0,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                scale: Vec3::new(25., 25., 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5));
}
