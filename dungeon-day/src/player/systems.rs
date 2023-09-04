use crate::board::components::Position;
use crate::parts::components::{Actor, Part};
use crate::player::components::Player;
use crate::utility::Location;
use bevy::prelude::*;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Actor::default(),
        Player,
        Part {
            kind: "Prince of Persia".to_string(),
        },
        Position {
            value: Location::new(0, 0),
        },
    ));
}
