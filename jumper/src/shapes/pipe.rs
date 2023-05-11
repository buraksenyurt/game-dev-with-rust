use crate::constants::STD_PIPE_WIDTH;
use crate::shapes::pipe_type::PipeType;
use bevy::prelude::{Color, Commands, Sprite, SpriteBundle, Transform, Vec3};
use bevy_rapier2d::prelude::{Collider, RigidBody};

pub struct Pipe {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub hegiht: f32,
    pub z_deep: f32,
    pub color: Color,
    pub pipe_type: PipeType,
}

pub fn spawn_pipe(commands: &mut Commands, p: Pipe) {
    let width = match p.pipe_type {
        PipeType::SMALL => STD_PIPE_WIDTH * 0.5,
        PipeType::MIDI => STD_PIPE_WIDTH,
        PipeType::BIG => STD_PIPE_WIDTH * 1.2,
    };
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: p.color,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(p.x, p.y, p.z),
                scale: Vec3::new(width, p.hegiht, p.z_deep),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5));
}
