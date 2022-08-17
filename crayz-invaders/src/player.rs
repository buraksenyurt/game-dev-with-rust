use crate::components::{FromPlayer, Movable, Player, SpriteSize, Velocity};
use crate::constant::*;
use crate::resources::{GameTextures, WinSize};
use crate::{Enemy, EnemyCount, ExplosionToSpawn, Laser};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::utils::HashSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, create_system)
            .add_system(keyboard_event_system)
            .add_system(fire_system);
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
        .insert(SpriteSize::from(PLAYER_SIZE))
        .insert(Movable { despawnable: false })
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

fn fire_system(
    mut commands: Commands,
    k: Res<Input<KeyCode>>,
    textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player) = query.get_single() {
        if k.just_pressed(KeyCode::Space) {
            let (x, y) = (player.translation.x, player.translation.y);

            commands
                .spawn_bundle(SpriteBundle {
                    texture: textures.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x, y + PLAYER_SIZE.1 * SPRITE_SCALE, 0.),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Laser)
                .insert(FromPlayer)
                .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                .insert(Movable { despawnable: true })
                .insert(Velocity { x: 0., y: 1. });
        }
    }
}

pub fn laser_hit_system(
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
