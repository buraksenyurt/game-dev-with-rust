use crate::ascii::{spawn_sprite, AsciiSheet};
use crate::TILE_SIZE;
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
    let sprite = spawn_sprite(
        &mut commands,
        &ascii_res,
        0,
        Color::GOLD,
        Vec3::new(2. * TILE_SIZE, -2. * TILE_SIZE, 900.),
    );

    commands.entity(sprite).insert(Player {
        movement_speed: 2.5,
    });
}
