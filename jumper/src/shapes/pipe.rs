use bevy::prelude::Color;
use crate::shapes::pipe_type::PipeType;

pub struct Pipe {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub hegiht: f32,
    pub z_deep: f32,
    pub color: Color,
    pub pipe_type: PipeType,
}