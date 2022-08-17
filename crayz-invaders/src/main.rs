mod components;
mod constant;
mod enemy;
mod player;
mod resources;

use crate::components::{
    Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, Laser, Movable, Velocity,
};
use crate::constant::*;
use crate::enemy::{enemy_hit_system, EnemyPlugin};
use crate::player::{laser_hit_system, PlayerPlugin};
use crate::resources::{EnemyCount, GameTextures, WinSize};
use bevy::prelude::*;

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
        .add_system(enemy_hit_system)
        .add_system(explosion_system)
        .add_system(explosion_animation_system)
        .run();
}

fn init_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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

    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32., 32.), 4, 3);
    let explosion = texture_atlases.add(texture_atlas);

    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion,
    };
    commands.insert_resource(game_textures);
    commands.insert_resource(EnemyCount(0));
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

fn explosion_system(
    mut commmands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>,
) {
    for (explosion_entity, explosion_spawn) in query.iter() {
        commmands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: game_textures.explosion.clone(),
                transform: Transform {
                    translation: explosion_spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(ExplosionTimer::default());

        commmands.entity(explosion_entity).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1;
            if sprite.index >= EXPLOSION_LENGTH {
                commands.entity(entity).despawn();
            }
        }
    }
}
