use crate::components::{Enemy, SpriteSize};
use crate::{EnemyCount, GameTextures, WinSize, ENEMY_SIZE, MAX_ENEMY, SPRITE_SCALE};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_create_system);
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
