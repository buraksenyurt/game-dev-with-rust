use crate::action::events::*;
use crate::graphics::events::GraphicsWaitEvent;
use crate::states::GameState;
use bevy::prelude::*;

pub fn start_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerInput);
}

pub fn end_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::None);
}

pub fn turn_update_start(
    mut next_state: ResMut<NextState<GameState>>,
    mut ev_tick: EventWriter<TickEvent>,
) {
    next_state.set(GameState::TurnUpdate);
    ev_tick.send(TickEvent);
}

pub fn tick(mut ev_wait: EventReader<GraphicsWaitEvent>, mut ev_tick: EventWriter<TickEvent>) {
    if ev_wait.iter().len() == 0 {
        ev_tick.send(TickEvent);
    }
}

pub fn turn_update_end(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerInput);
}

pub fn turn_update_cancel(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerInput);
}
