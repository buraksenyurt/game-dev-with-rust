use crate::components::{Enemy, FromEnemy, Player, SpriteSize};
use crate::enemy_formation::{Formation, FormationMaker};
use crate::player::PlayerState;
use crate::{
    EnemyCount, ExplosionToSpawn, GameTextures, Laser, Movable, Velocity, WinSize,
    ENEMY_LASER_SIZE, ENEMY_SIZE, FPS, MAX_ENEMY, SPRITE_SCALE,
};
use bevy::ecs::schedule::ShouldRun;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::time::FixedTimestep;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FormationMaker::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.75))
                    .with_system(enemy_create_system),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(enemy_fire_criteria)
                    .with_system(enemy_fire_system),
            )
            .add_system(enemy_movement_system);
    }
}

fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for &t in enemy_query.iter() {
        let (x, y) = (t.translation.x, t.translation.y);

        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.enemy_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y - 15., 0.),
                    rotation: Quat::from_rotation_x(PI),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))
            .insert(FromEnemy)
            .insert(Movable { despawnable: true })
            .insert(Velocity { x: 0., y: -1. });
    }
}

fn enemy_fire_criteria() -> ShouldRun {
    if thread_rng().gen_bool(1. / 60.) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn enemy_create_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    window_size: Res<WinSize>,
    mut enemy_count: ResMut<EnemyCount>,
    mut formation_maker: ResMut<FormationMaker>,
) {
    if enemy_count.0 < MAX_ENEMY {
        let formation = formation_maker.build(&window_size);
        let start_point = formation.start;

        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(start_point.x, start_point.y, 10.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(formation)
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}

pub fn enemy_hit_system(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
) {
    if let Ok((ply_entity, ply_tf, ply_size)) = player_query.get_single() {
        let player_scale = Vec2::from(ply_tf.scale.xy());

        for (las_entity, las_tf, las_size) in laser_query.iter() {
            let laser_scale = Vec2::from(las_tf.scale.xy());

            let collision = collide(
                las_tf.translation,
                las_size.0 * laser_scale,
                ply_tf.translation,
                ply_size.0 * player_scale,
            );

            if let Some(_) = collision {
                commands.entity(ply_entity).despawn();
                player_state.shot(time.seconds_since_startup());
                commands.entity(las_entity).despawn();
                commands
                    .spawn()
                    .insert(ExplosionToSpawn(ply_tf.translation.clone()));

                break;
            }
        }
    }
}

fn enemy_movement_system(mut query: Query<(&mut Transform, &mut Formation), With<Enemy>>) {
    for (mut transform, mut formation) in query.iter_mut() {
        let (x_org, y_org) = (transform.translation.x, transform.translation.y);
        let max_distance = FPS * formation.speed;

        let dir: f32 = if formation.start.x < 0. { 1. } else { -1. };
        let pivot = formation.pivot;
        let radius = formation.radius;

        let angle =
            formation.angle + dir * formation.speed * FPS / (radius.x.min(radius.y) * PI / 2.);

        let x_dst = radius.x * angle.cos() + pivot.x;
        let y_dst = radius.y * angle.sin() + pivot.y;

        let dx = x_org - x_dst;
        let dy = y_org - y_dst;
        let distance = (dx * dx + dy * dy).sqrt();
        let distance_ratio = if distance != 0. {
            max_distance / distance
        } else {
            0.
        };

        let x = x_org - dx * distance_ratio;
        let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };
        let y = y_org - dy * distance_ratio;
        let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

        if distance < max_distance * formation.speed / 20. {
            formation.angle = angle;
        }

        let translation = &mut transform.translation;
        (translation.x, translation.y) = (x, y);
    }
}
