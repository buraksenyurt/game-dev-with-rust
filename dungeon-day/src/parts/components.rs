use crate::action::Action;
use bevy::prelude::*;

#[derive(Component)]
pub struct Part {
    pub kind: String,
}

#[derive(Component, Default)]
pub struct Actor(pub Option<Box<dyn Action>>);

#[derive(Component)]
pub struct Walk;
