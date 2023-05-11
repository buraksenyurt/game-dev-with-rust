use crate::constants::PLAYER_VELOCITY;
use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;

pub fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    let mut player = query.single_mut();
    let mut translation = Vec2::new(0.0, 0.0);
    if input.pressed(KeyCode::Right) || input.pressed(KeyCode::D) {
        translation.x += time.delta_seconds() * PLAYER_VELOCITY;
    }
    if input.pressed(KeyCode::Left) || input.pressed(KeyCode::A) {
        translation.x += time.delta_seconds() * PLAYER_VELOCITY * -1.0;
    }
    player.translation = Some(translation);
}
