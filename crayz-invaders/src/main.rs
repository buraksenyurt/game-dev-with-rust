mod components;
mod constant;
mod enemy;
mod player;
mod resources;

use crate::components::{Enemy, FromPlayer, Laser, Movable, SpriteSize, Velocity};
use crate::constant::*;
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;
use crate::resources::{GameTextures, WinSize};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Crayz Invaders".to_string(),
            width: 480.0,
            height: 640.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(init_system)
        .add_system(movement_system)
        .add_system(laser_hit_system)
        .run();
}

fn init_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let main_window = windows.get_primary_mut().unwrap();
    main_window.set_position(IVec2::new(0, 0));

    let (w_width, w_height) = (main_window.width(), main_window.height());
    let window_size = WinSize {
        width: w_width,
        height: w_height,
    };
    commands.insert_resource(window_size);

    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
    };
    commands.insert_resource(game_textures);
}

fn movement_system(
    mut commands: Commands,
    window_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (e, v, mut t, mov) in query.iter_mut() {
        let translation = &mut t.translation;
        translation.x += v.x * FPS * BASE_SPEED;
        translation.y += v.y * FPS * BASE_SPEED;

        if mov.despawnable {
            const MARGIN: f32 = 200.;
            if translation.y > window_size.height / 2. + MARGIN
                || translation.y < -window_size.height / 2. - MARGIN
                || translation.x > window_size.width / 2. + MARGIN
                || translation.x < -window_size.width / 2. - MARGIN
            {
                commands.entity(e).despawn();
            }
        }
    }
}

fn laser_hit_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    for (laser_entity, laser_transform, laser_size) in laser_query.iter() {
        let laser_scale = Vec2::from(laser_transform.scale.xy());

        for (enemy_entity, enemy_transform, enemy_size) in enemy_query.iter() {
            let enemy_scale = Vec2::from(enemy_transform.scale.xy());

            let collision = collide(
                laser_transform.translation,
                laser_size.0 * laser_scale,
                enemy_transform.translation,
                enemy_size.0 * enemy_scale,
            );

            if let Some(_) = collision {
                commands.entity(enemy_entity).despawn();
                commands.entity(laser_entity).despawn();
            }
        }
    }
}
