use bevy::prelude::{Color, Commands, Sprite, SpriteBundle, Transform, Vec3};
use bevy_rapier2d::prelude::{Collider, RigidBody};

pub struct Pipe {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub hegiht: f32,
    pub z_deep: f32,
    pub color: Color,
}

pub fn spawn_pipe(commands: &mut Commands, p: Pipe) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: p.color,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(p.x, p.y, p.z),
                scale: Vec3::new(p.width, p.hegiht, p.z_deep),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5));
}
