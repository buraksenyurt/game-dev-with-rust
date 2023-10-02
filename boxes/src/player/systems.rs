use crate::board::components::*;
use crate::board::resources::ActiveBoard;
use crate::components::*;
use crate::globals::*;
use crate::player::components::Player;
use bevy::prelude::*;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Part {
            kind: TileKind::Player,
        },
        Position {
            value: Vector::new(0, 0),
        },
    ));
}

pub fn move_player(
    keys: ResMut<Input<KeyCode>>,
    mut player_query: Query<(Entity, &mut Position), With<Player>>,
    board: Res<ActiveBoard>,
) {
    let Ok((_entity,mut position)) = player_query.get_single_mut() else { return };
    for (key, dir) in KEY_DIRECTION_MAP {
        if !keys.just_pressed(key) {
            continue;
        }
        let new_position = position.value + dir;

        if board.tiles.contains_key(&new_position) {
            let next_tile_kind = board.tiles[&new_position].1;
            if next_tile_kind == TileKind::Grass {
                position.value = new_position;
            }
        }
    }
}
