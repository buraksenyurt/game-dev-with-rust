use crate::asset::resources::AssetList;
use crate::states::AppState;
use bevy::asset::LoadState;
use bevy::prelude::*;

pub fn check_assets_load(
    assets: Res<AssetList>,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    match asset_server.get_group_load_state(assets.0.iter().map(|a| a.id())) {
        LoadState::Loaded => app_state.set(AppState::Game),
        LoadState::Failed => {
            error!("Asset'ler yüklenemiyor");
        }
        _ => {}
    }
}
