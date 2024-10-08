pub mod events;
pub mod models;
mod systems;

use crate::action::events::*;
use crate::action::systems::{execute_actions, populate_actor_queue, prepare_walk};
use crate::states::GameState;
use bevy::prelude::*;
use std::collections::VecDeque;

pub trait Action: Send + Sync {
    fn apply(&self, world: &mut World) -> bool;
}

#[derive(Default, Resource)]
pub struct ActorQueue(pub VecDeque<Entity>);

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()
            .add_systems(
                Update,
                (
                    execute_actions.run_if(on_event::<TickEvent>()),
                    prepare_walk.run_if(on_event::<NextActorEvent>()),
                ),
            )
            .add_systems(OnExit(GameState::PlayerInput), populate_actor_queue);
    }
}
