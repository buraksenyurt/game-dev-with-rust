use crate::state::GameState;
use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameSystemSet {
    PlayerInput,
    EntityUpdates,
    CollisionDetections,
    DespawnEntities,
}
pub struct PlannerPlugin;
impl Plugin for PlannerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GameSystemSet::PlayerInput,
                GameSystemSet::EntityUpdates,
                GameSystemSet::CollisionDetections,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(GameSystemSet::DespawnEntities)
                .before(GameSystemSet::PlayerInput),
        );
    }
}
