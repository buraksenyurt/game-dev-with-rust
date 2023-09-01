use crate::board::components::{Position, Tile};
use crate::board::resources::ActiveBoard;
use crate::utility::Location;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn spawn_board_map(mut commands: Commands, mut board: ResMut<ActiveBoard>) {
    board.tiles = HashMap::new();
    for row in 0..16 {
        for column in 0..16 {
            let value = Location::new(row, column);
            let tile = commands.spawn((Position { value }, Tile)).id();
            board.tiles.insert(value, tile);
        }
    }
}
