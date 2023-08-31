use crate::game::live_data::resources::LiveData;
use crate::game::live_data::systems::*;
use crate::AppState;
use bevy::app::App;
use bevy::prelude::*;

pub mod resources;
mod systems;

pub const DEFAULT_FUEL_LEVEL: f32 = 200.;

pub struct LiveDataPlugin;
impl Plugin for LiveDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LiveData>()
            .add_systems(OnEnter(AppState::Game), prepare_live_data)
            .add_systems(Update, update_live_data.run_if(in_state(AppState::Game)))
            //.add_systems(OnExit(AppState::Game), remove_live_data)
        ;
    }
}
