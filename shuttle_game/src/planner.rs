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
                .chain(),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(GameSystemSet::DespawnEntities)
                .before(GameSystemSet::PlayerInput),
        );
    }
}
