use crate::global::{Difficulty, GameState};
use bevy::prelude::*;

pub fn game_setup_system(mut _commands: Commands, difficulty: Res<Difficulty>) {
    info!("Güncel zorluk seviyesi, {:?}", difficulty.0);
}

pub fn playing_system(time: Res<Time>, mut _game_state: ResMut<NextState<GameState>>) {}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn soldier_animation_system(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

pub fn soldier_setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut texture_handle = asset_server.load("soldier_run_right.png");
    if input.just_pressed(KeyCode::ArrowLeft) {
        texture_handle = asset_server.load("soldier_run_left.png");
    } else if input.just_pressed(KeyCode::ArrowRight) {
        texture_handle = asset_server.load("soldier_run_right.png");
    }
    info!("{:?} kullanılıyor...", texture_handle);

    let layout = TextureAtlasLayout::from_grid(UVec2::new(50, 50), 7, 1, None, None);
    let texture_atlas_layout_handle = texture_atlases.add(layout);
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn((
        Sprite {
            image: texture_handle,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout_handle,
                index: animation_indices.first,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(6.)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

