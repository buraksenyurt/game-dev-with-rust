use crate::ascii::{spawn_sprite, AsciiSheet};
use crate::game_state::GameState;
use crate::tile_map::{EncounterBuilder, TileCollider};
use crate::TILE_SIZE;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

#[derive(Component)]
pub struct Player {
    movement_speed: f32,
    just_moved: bool,
}

#[derive(Component)]
pub struct EncounterTrucker {
    timer: Timer,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Overworld).with_system(show))
            .add_system_set(SystemSet::on_exit(GameState::Overworld).with_system(hide))
            .add_system_set(
                SystemSet::on_update(GameState::Overworld)
                    .with_system(on_combat.after("movement"))
                    .with_system(camera_follow.after("movement"))
                    .with_system(movement.label("movement")),
            )
            .add_startup_system(spawn);
    }
}

fn movement(
    mut query: Query<(&mut Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut transform) = query.single_mut();
    player.just_moved = false;
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
        if x_delta != 0. {
            player.just_moved = true;
        }
        transform.translation = target;
    }
    let target = transform.translation + Vec3::new(0., y_delta, 0.);
    if !wall_query
        .iter()
        .any(|&transform| is_collision_with_wall(target, transform.translation))
    {
        if y_delta != 0. {
            player.just_moved = true;
        }
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

fn on_combat(
    mut player_query: Query<(&Player, &mut EncounterTrucker, &Transform)>,
    enc_query: Query<&Transform, (With<EncounterBuilder>, Without<Player>)>,
    mut state: ResMut<State<GameState>>,
    time: Res<Time>,
) {
    let (player, mut encounter_tracker, player_transform) = player_query.single_mut();
    let player_translation = player_transform.translation;

    if player.just_moved
        && enc_query
            .iter()
            .any(|&transform| is_collision_with_wall(player_translation, transform.translation))
    {
        encounter_tracker.timer.tick(time.delta());
        if encounter_tracker.timer.just_finished() {
            info!("Oyuncu savaş bölgesinde");
            state.set(GameState::Combat).expect("State değiştirilemedi");
        }
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

    commands
        .entity(sprite)
        .insert(Player {
            movement_speed: 2.5,
            just_moved: false,
        })
        .insert(EncounterTrucker {
            timer: Timer::from_seconds(1., TimerMode::Repeating),
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

fn show(
    mut player_query: Query<&mut Visibility, With<Player>>,
    child_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let mut player_vis = player_query.single_mut();
    player_vis.is_visible = true;
    if let Ok(children) = child_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = true;
            }
        }
    }
}

fn hide(
    mut player_query: Query<&mut Visibility, With<Player>>,
    child_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let mut player_vis = player_query.single_mut();
    player_vis.is_visible = false;
    if let Ok(children) = child_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = false;
            }
        }
    }
}
