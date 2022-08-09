use crate::components::{Player, Velocity};
use crate::constant::*;
use crate::resources::{GameTextures, WinSize};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, create_system)
            .add_system(movement_system)
            .add_system(keyboard_event_system);
    }
}

fn create_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    window_size: Res<WinSize>,
) {
    let bottom = -window_size.height / 2.;

    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity { x: 1., y: 0. });
}

fn keyboard_event_system(k: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    if let Ok(mut v) = query.get_single_mut() {
        v.x = if k.pressed(KeyCode::A) {
            -1.
        } else if k.pressed(KeyCode::D) {
            1.
        } else {
            0.
        };

        v.y = if k.pressed(KeyCode::S) {
            -1.
        } else if k.pressed(KeyCode::W) {
            1.
        } else {
            0.
        };
    }
}

fn movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (v, mut t) in query.iter_mut() {
        let translation = &mut t.translation;
        translation.x += v.x * FPS * BASE_SPEED;
        translation.y += v.y * FPS * BASE_SPEED;
    }
}
