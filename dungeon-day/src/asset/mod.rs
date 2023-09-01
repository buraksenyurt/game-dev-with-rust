use crate::asset::resources::AssetList;
use crate::asset::systems::check_assets_load;
use crate::states::AppState;
use bevy::app::*;
use bevy::prelude::*;

mod resources;
mod systems;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>().add_systems(
            Update,
            check_assets_load.run_if(in_state(AppState::LoadAssets)),
        );
    }
}
