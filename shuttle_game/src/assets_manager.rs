use bevy::prelude::*;

#[derive(Debug, Resource, Default)]
pub struct AssetsResource {
    pub pickup_crate: Handle<Scene>,
    pub shuttle: Handle<Scene>,
    pub rocket: Handle<Scene>,
    pub planets: Vec<Handle<Scene>>,
    pub score_font: Handle<Font>,
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
        planets: vec![
            asset_server.load("planet_1.glb#Scene0"),
            asset_server.load("planet_2.glb#Scene0"),
            asset_server.load("planet_3.glb#Scene0"),
            asset_server.load("planet_4.glb#Scene0"),
        ],
        score_font: asset_server.load("FiraMono-Medium.ttf"),
    }
}
