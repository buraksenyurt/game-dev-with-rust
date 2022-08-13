mod components;
mod constant;
mod enemy;
mod player;
mod resources;

use crate::components::{
    Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromPlayer, Laser, Movable, SpriteSize,
    Velocity,
};
use crate::constant::*;
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;
use crate::resources::{EnemyCount, GameTextures, WinSize};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::utils::HashSet;

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

fn laser_hit_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    for (laser_entity, laser_transform, laser_size) in laser_query.iter() {
        if despawned_entities.contains(&laser_entity) {
            continue;
        }
        let laser_scale = Vec2::from(laser_transform.scale.xy());

        for (enemy_entity, enemy_transform, enemy_size) in enemy_query.iter() {
            if despawned_entities.contains(&enemy_entity)
                || despawned_entities.contains(&laser_entity)
            {
                continue;
            }
            let enemy_scale = Vec2::from(enemy_transform.scale.xy());

            let collision = collide(
                laser_transform.translation,
                laser_size.0 * laser_scale,
                enemy_transform.translation,
                enemy_size.0 * enemy_scale,
            );

            if let Some(_) = collision {
                commands.entity(enemy_entity).despawn();
                despawned_entities.insert(enemy_entity);
                enemy_count.0 -= 1;

                commands.entity(laser_entity).despawn();
                despawned_entities.insert(laser_entity);

                commands
                    .spawn()
                    .insert(ExplosionToSpawn(enemy_transform.translation.clone()));

                break;
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
