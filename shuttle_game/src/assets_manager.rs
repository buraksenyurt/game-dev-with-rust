use bevy::prelude::*;

#[derive(Debug, Resource, Default)]
pub struct AssetsResource {
    pub pickup_crate: Handle<Scene>,
    pub shuttle: Handle<Scene>,
    pub rocket: Handle<Scene>,
}

pub struct AssetsManagerPlugin;

impl Plugin for AssetsManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetsResource>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut assets: ResMut<AssetsResource>, asset_server: Res<AssetServer>) {
    *assets = AssetsResource {
        pickup_crate: asset_server.load("crate.glb#Scene0"),
        shuttle: asset_server.load("shuttle.glb#Scene0"),
        rocket: asset_server.load("missile.glb#Scene0"),
    }
}
