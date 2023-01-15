use crate::ascii::{AsciiSheet, spawn_sprite};
use crate::game_state::GameState;
use bevy::prelude::*;

pub struct CombatPlugin;

#[derive(Component)]
pub struct Enemy;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Combat).with_system(combat_camera))
            .add_system_set(SystemSet::on_enter(GameState::Combat).with_system(spawn_enemy))
            .add_system_set(SystemSet::on_exit(GameState::Combat).with_system(despawn_enemy));
    }
}

fn spawn_enemy(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let sprite = spawn_sprite(
        &mut commands,
        &ascii,
        'e' as usize,
        Color::RED,
        Vec3::new(0.0, 0.5, 100.0),
    );
    commands
        .entity(sprite)
        .insert(Enemy)
        .insert(Name::new("Eagle"));
}

fn despawn_enemy(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn combat_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}
