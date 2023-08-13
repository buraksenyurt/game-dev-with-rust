use crate::shapes::pipe_type::PipeType;
use bevy::prelude::Color;

pub struct Pipe {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub hegiht: f32,
    pub z_deep: f32,
    pub color: Color,
    pub pipe_type: PipeType,
}
