use crate::planner::GameSystemSet;
use bevy::app::{App, Plugin, Update};
use bevy::log::info;
use bevy::prelude::{Entity, IntoSystemConfigs, Query, Transform};

pub struct LogPlugin;

impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, log_position.after(GameSystemSet::EntityUpdates));
    }
}
fn log_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!("Entity {:?} at {:?}", entity, transform.translation);
    }
}
