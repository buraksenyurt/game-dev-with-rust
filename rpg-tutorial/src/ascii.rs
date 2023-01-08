use bevy::prelude::*;

pub struct AsciiPlugin;

#[derive(Resource)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load);
    }
}

fn load(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let ascii_img = assets_server.load("ascii_map2.png");
    let atlas = TextureAtlas::from_grid(ascii_img, Vec2::splat(32.), 16, 16, None, None);
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}
