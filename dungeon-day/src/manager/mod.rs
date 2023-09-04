use crate::action::events::*;
use crate::input::events::PlayerInputReadyEvent;
use crate::manager::systems::*;
use crate::states::{AppState, GameState};
use bevy::prelude::*;

mod systems;

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), start_game)
            .add_systems(OnExit(AppState::Game), end_game)
            .add_systems(
                Update,
                (
                    turn_update_start.run_if(on_event::<PlayerInputReadyEvent>()),
                    turn_update_end.run_if(on_event::<ActionsCompleteEvent>()),
                    turn_update_cancel.run_if(on_event::<InvalidPlayerActionEvent>()),
                ),
            )
            .add_systems(Update, tick.in_set(GameState::TurnUpdate));
    }
}
