use crate::shapes::pipe::Pipe;
use crate::shapes::pipe_type::PipeType;
use bevy::prelude::{Bundle, Sprite, SpriteBundle, Transform, Vec3};
use bevy_rapier2d::prelude::{Collider, RigidBody};

#[derive(Bundle)]
pub struct PipeBundle {
    sprite: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl PipeBundle {
    pub fn new(p: Pipe) -> Self {
        let width = match p.pipe_type {
            PipeType::Small(s) => s,
            PipeType::Midi(s) => s,
            PipeType::Big(s) => s,
        };

        Self {
            sprite: SpriteBundle {
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
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
        }
    }
}
