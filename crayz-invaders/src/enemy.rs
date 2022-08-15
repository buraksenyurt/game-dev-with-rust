use crate::components::{Enemy, FromEnemy, SpriteSize};
use crate::{
    EnemyCount, GameTextures, Laser, Movable, Velocity, WinSize, ENEMY_LASER_SIZE, ENEMY_SIZE,
    MAX_ENEMY, SPRITE_SCALE,
};
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.75))
                .with_system(enemy_create_system),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(enemy_fire_criteria)
                .with_system(enemy_fire_system),
        );
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
) {
    if enemy_count.0 < MAX_ENEMY {
        let mut rnd = thread_rng();
        let w = window_size.width / 2. - 100.;
        let h = window_size.height / 2. - 150.;
        let x = rnd.gen_range(-w..w);
        let y = rnd.gen_range(-h..h);

        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}
