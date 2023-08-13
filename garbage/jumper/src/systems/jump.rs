use crate::constants::{MAX_JUMP_DISTANCE, PLAYER_JUMP_VELOCTITY};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};

#[derive(Component)]
pub struct Jump(f32);

pub fn jump_to(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();
    if (input.pressed(KeyCode::Up) || input.pressed(KeyCode::W)) && output.grounded {
        info!("Sıçrama yükleniyor...");
        commands.entity(player).insert(Jump(0.));
    }
}

pub fn rise_high(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
    if query.is_empty() {
        return;
    }
    let (entity, mut player, mut jump) = query.single_mut();
    let mut mov = time.delta().as_secs_f32() * PLAYER_JUMP_VELOCTITY;
    if mov + jump.0 >= MAX_JUMP_DISTANCE {
        info!("Sıçrama konumunda...");
        mov = MAX_JUMP_DISTANCE - jump.0;
        commands.entity(entity).remove::<Jump>();
    }
    jump.0 += mov;

    match player.translation {
        Some(v) => player.translation = Some(Vec2::new(v.x, mov)),
        None => player.translation = Some(Vec2::new(0., mov)),
    }
}

pub fn fall_down(
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController, Without<Jump>>,
) {
    if query.is_empty() {
        return;
    }

    let mut player = query.single_mut();
    let mov = time.delta().as_secs_f32() * (PLAYER_JUMP_VELOCTITY * 0.75) * -1.;
    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, mov)),
        None => player.translation = Some(Vec2::new(0.0, mov)),
    }
}
