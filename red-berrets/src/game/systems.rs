use crate::global::{Difficulty, GameState};
use bevy::prelude::*;

pub fn game_setup_system(mut _commands: Commands, difficulty: Res<Difficulty>) {
    info!("Güncel zorluk seviyesi, {:?}", difficulty.0);
}

pub fn playing_system(time: Res<Time>, mut _game_state: ResMut<NextState<GameState>>) {}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn soldier_animation_system(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn soldier_setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    input: Res<Input<KeyCode>>,
) {
    let mut texture_handle = Default::default();
    if input.just_pressed(KeyCode::Left) {
        texture_handle = asset_server.load("soldier_run_left.png");
    } else if input.just_pressed(KeyCode::Right) {
        texture_handle = asset_server.load("soldier_run_right.png");
    }
    info!("{:?} kullanılıyor...", texture_handle);

    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(50., 50.), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
