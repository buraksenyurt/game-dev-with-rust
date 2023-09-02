use crate::board::components::Position;
use crate::input::KEY_DIRECTION_MAP;
use crate::player::components::Player;
use bevy::prelude::*;

pub fn update_player_position(
    keys: ResMut<Input<KeyCode>>,
    mut player_query: Query<&mut Position, With<Player>>,
) {
    let Ok(mut position) = player_query.get_single_mut() else { return };
    for (key, direction) in KEY_DIRECTION_MAP {
        if !keys.just_pressed(key) {
            continue;
        }
        position.value += direction;
    }
}
