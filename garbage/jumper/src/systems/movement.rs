use crate::constants::PLAYER_VELOCITY;
use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;

pub fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    let mut player = query.single_mut();
    let mut mov = 0.;
    if input.pressed(KeyCode::Right) || input.pressed(KeyCode::D) {
        mov += time.delta_seconds() * PLAYER_VELOCITY;
    }
    if input.pressed(KeyCode::Left) || input.pressed(KeyCode::A) {
        mov += time.delta_seconds() * PLAYER_VELOCITY * -1.0;
    }

    match player.translation {
        Some(v) => player.translation = Some(Vec2::new(mov, v.y)),
        None => player.translation = Some(Vec2::new(mov, 0.)),
    }
}
