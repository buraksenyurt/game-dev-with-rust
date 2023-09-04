use crate::action::models::*;
use crate::action::ActorQueue;
use crate::board::components::*;
use crate::input::events::*;
use crate::input::KEY_DIRECTION_MAP;
use crate::parts::components::*;
use crate::player::components::*;
use bevy::prelude::*;
use std::collections::VecDeque;

pub fn update_player_position(
    keys: ResMut<Input<KeyCode>>,
    mut player_query: Query<(Entity, &Position, &mut Actor), With<Player>>,
    mut queue: ResMut<ActorQueue>,
    mut ev_input: EventWriter<PlayerInputReadyEvent>,
) {
    let Ok((entity, position,mut actor)) = player_query.get_single_mut() else { return };
    for (key, direction) in KEY_DIRECTION_MAP {
        if !keys.just_pressed(key) {
            continue;
        }
        let action = WalkAction {
            entity,
            loc: position.value + direction,
        };
        actor.0 = Some(Box::new(action));
        queue.0 = VecDeque::from([entity]);
        ev_input.send(PlayerInputReadyEvent);
    }
}
