use crate::game::live_data::resources::LiveData;
use bevy::app::App;
use bevy::prelude::Plugin;

pub mod resources;
pub const DEFAULT_FUEL_LEVEL: f32 = 200.;

pub struct LiveDataPlugin;
impl Plugin for LiveDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LiveData>();
    }
}
