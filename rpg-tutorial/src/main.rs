mod player;
mod ascii;

use crate::player::PlayerPlugin;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub const SIZE: (f32, f32) = (1600., 900.);
pub const RESOLUTION: f32 = SIZE.0 / SIZE.1;
pub const TILE_SIZE: f32 = 0.1;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "RPG Tutorial".to_string(),
                width: SIZE.0,
                height: SIZE.1,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_startup_system_set(SystemSet::new().with_system(setup))
        .add_plugin(PlayerPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            top: 1.,
            bottom: -1.,
            right: 1. * RESOLUTION,
            left: -1. * RESOLUTION,
            scaling_mode: ScalingMode::None,
            ..default()
        },
        ..default()
    });
}

#[derive(Resource)]
struct AsciiSheet(Handle<TextureAtlas>);

fn load_ascii(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let ascii_img = assets_server.load("ascii_map2.png");
    let atlas = TextureAtlas::from_grid(ascii_img, Vec2::splat(32.), 16, 16, None, None);
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}
