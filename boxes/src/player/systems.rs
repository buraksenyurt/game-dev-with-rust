use crate::board::components::{Part, Position};
use crate::components::Vector;
use crate::player::components::Player;
use bevy::prelude::*;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Part {
            kind: "Prince of Persia".to_string(),
        },
        Position {
            value: Vector::new(0, 0),
        },
    ));
}
