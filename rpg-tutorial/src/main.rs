mod ascii;
mod combat;
mod game_state;
mod player;
mod tile_map;

use crate::ascii::AsciiPlugin;
use crate::combat::CombatPlugin;
use crate::game_state::GameState;
use crate::player::PlayerPlugin;
use crate::tile_map::TileMapPlugin;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub const SIZE: (f32, f32) = (1600., 900.);
pub const RESOLUTION: f32 = SIZE.0 / SIZE.1;
pub const TILE_SIZE: f32 = 0.1;

fn main() {
    App::new()
        .add_state(GameState::Overworld)
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
        .add_plugin(AsciiPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(CombatPlugin)
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
