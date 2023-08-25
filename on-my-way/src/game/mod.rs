use crate::game::resources::GameState;
use bevy::app::App;
use bevy::prelude::Plugin;

pub mod resources;
pub const DEFAULT_FUEL_LEVEL: f32 = 200.;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>();
    }
}
