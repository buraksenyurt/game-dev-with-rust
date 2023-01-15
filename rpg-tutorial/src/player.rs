use crate::ascii::{spawn_sprite, AsciiSheet};
use crate::game_state::GameState;
use crate::tile_map::{EncounterBuilder, TileCollider};
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
        app.add_system_set(
            SystemSet::on_update(GameState::Overworld)
                .with_system(encounter_check)
                .with_system(camera_follow)
                .with_system(movement),
        )
        .add_startup_system(spawn);
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
    if !wall_query
        .iter()
        .any(|&transform| is_collision_with_wall(target, transform.translation))
    {
        transform.translation = target;
    }
    let target = transform.translation + Vec3::new(0., y_delta, 0.);
    if !wall_query
        .iter()
        .any(|&transform| is_collision_with_wall(target, transform.translation))
    {
        transform.translation = target;
    }
}

fn is_collision_with_wall(player_pos: Vec3, wall_translation: Vec3) -> bool {
    let collision = collide(
        player_pos,
        Vec2::splat(TILE_SIZE * 1.),
        wall_translation,
        Vec2::splat(TILE_SIZE * 0.9),
    );
    collision.is_some()
}

fn encounter_check(
    query: Query<&Transform, With<Player>>,
    enc_query: Query<&Transform, (With<EncounterBuilder>, Without<Player>)>,
    mut state: ResMut<State<GameState>>,
) {
    let player_translation = query.single().translation;
    if enc_query
        .iter()
        .any(|&transform| is_collision_with_wall(player_translation, transform.translation))
    {
        info!("Oyuncu savaş bölgesine girdi");
        state.set(GameState::Combat).expect("State değiştirilemedi");
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

fn camera_follow(
    query: Query<&Transform, With<Player>>,
    mut cam_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let transform = query.single();
    let mut camera_transform = cam_query.single_mut();
    camera_transform.translation.x = transform.translation.x;
    camera_transform.translation.y = transform.translation.y;
}
