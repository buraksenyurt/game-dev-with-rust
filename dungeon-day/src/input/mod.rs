mod systems;

use crate::input::systems::*;
use crate::utility::Location;
use bevy::prelude::*;

const KEY_DIRECTION_MAP: [(KeyCode, Location); 4] = [
    (KeyCode::Up, Location::UP),
    (KeyCode::Down, Location::DOWN),
    (KeyCode::Left, Location::LEFT),
    (KeyCode::Right, Location::RIGHT),
];

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_player_position);
    }
}
