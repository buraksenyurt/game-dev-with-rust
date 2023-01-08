use crate::{AsciiSheet, TILE_SIZE};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    movement_speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn).add_system(movement);
    }
}

fn movement(
    mut query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = query.single_mut();

    if keyboard.pressed(KeyCode::Up) {
        transform.translation.y += TILE_SIZE * player.movement_speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Down) {
        transform.translation.y -= TILE_SIZE * player.movement_speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Left) {
        transform.translation.x -= TILE_SIZE * player.movement_speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Right) {
        transform.translation.x += TILE_SIZE * player.movement_speed * time.delta_seconds();
    }
}

fn spawn(mut commands: Commands, ascii_res: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.color = Color::GOLD;
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    commands.spawn((
        SpriteSheetBundle {
            sprite,
            texture_atlas: ascii_res.0.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 100.),
                ..default()
            },
            ..default()
        },
        Player {
            movement_speed: 2.5,
        },
    ));
}
