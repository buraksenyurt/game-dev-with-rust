use crate::ascii::{spawn_sprite, AsciiSheet};
use crate::tile_map::TileCollider;
use crate::TILE_SIZE;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

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
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = query.single_mut();
    let (mut y_delta, mut x_delta) = (0., 0.);
    if keyboard.pressed(KeyCode::Up) {
        y_delta += TILE_SIZE * player.movement_speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Down) {
        y_delta -= TILE_SIZE * player.movement_speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Left) {
        x_delta -= TILE_SIZE * player.movement_speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Right) {
        x_delta += TILE_SIZE * player.movement_speed * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(x_delta, 0., 0.);
    if is_collision_with_wall(target, &wall_query) {
        transform.translation = target;
    }
    let target = transform.translation + Vec3::new(0., y_delta, 0.);
    if is_collision_with_wall(target, &wall_query) {
        transform.translation = target;
    }
}

fn is_collision_with_wall(
    player_pos: Vec3,
    query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for transform in query.iter() {
        let collision = collide(
            player_pos,
            Vec2::splat(TILE_SIZE * 1.),
            transform.translation,
            Vec2::splat(TILE_SIZE * 0.9),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
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
