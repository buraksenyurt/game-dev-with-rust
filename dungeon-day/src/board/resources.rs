use crate::utility::Location;
use bevy::prelude::{Entity, Resource};
use std::collections::HashMap;

#[derive(Default, Resource)]
pub struct ActiveBoard {
    pub tiles: HashMap<Location, Entity>,
}
