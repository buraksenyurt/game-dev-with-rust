mod components;
mod constant;
mod enemy;
mod enemy_formation;
mod player;
mod point;
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
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Crayz Invaders".to_string(),
                resolution: (480., 640.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((PlayerPlugin, EnemyPlugin))
        .add_systems(Startup, init_system)
        .add_systems(
            Update,
            (
                movement_system,
                laser_hit_system,
                enemy_hit_system,
                explosion_system,
                explosion_animation_system,
            ),
        )
        .run();
}

fn init_system(
    mut commands: Commands,
    window_query: Query<&Window>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let window = window_query.single();
    let (w_width, w_height) = (window.width(), window.height());
    commands.insert_resource(WinSize {
        width: w_width,
        height: w_height,
    });

    let explosion_layout =
        TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 3, None, None);
    let explosion_layout_handle = texture_atlas_layouts.add(explosion_layout);

    commands.insert_resource(GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion: asset_server.load(EXPLOSION_SHEET),
        explosion_layout: explosion_layout_handle,
    });
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
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>,
) {
    for (explosion_entity, explosion_spawn) in query.iter() {
        commands.spawn((
            Sprite {
                image: game_textures.explosion.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: game_textures.explosion_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Transform {
                translation: explosion_spawn.0,
                ..Default::default()
            },
            Explosion,
            ExplosionTimer::default(),
        ));
        commands.entity(explosion_entity).despawn();
    }
}

fn explosion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut Sprite), With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            if let Some(ref mut atlas) = sprite.texture_atlas {
                atlas.index += 1;
                if atlas.index >= EXPLOSION_LENGTH {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
