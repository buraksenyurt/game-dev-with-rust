use bevy::prelude::*;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Difficulty(pub Level);

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Level {
    Training,
    Normal,
    Brutal,
}
