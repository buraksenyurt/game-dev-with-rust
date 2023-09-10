use crate::board::components::Position;
use crate::parts::components::*;
use crate::utility::Location;
use bevy::prelude::*;
use rand::Rng;

pub fn spawn_non_player_characters(mut commands: Commands) {
    for _ in 0..3 {
        let (x, y) = (
            rand::thread_rng().gen_range(0..16),
            rand::thread_rng().gen_range(0..16),
        );
        commands.spawn((
            Actor::default(),
            Part {
                kind: "NPC".to_string(),
            },
            Position {
                value: Location::new(x, y),
            },
            Walk,
        ));
    }
}
