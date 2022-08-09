use crate::constant::*;
use crate::resources::{GameTextures, WinSize};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, create_player_system);
    }
}

fn create_player_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    window_size: Res<WinSize>,
) {
    let bottom = -window_size.height / 2.;

    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
