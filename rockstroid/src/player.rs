use crate::constant::{PLAYER_PUSH_VALUE, PLAYER_SHOT_TIME, PLAYER_TURN_RATE, SHOT_SPEED};
use crate::fermat::angle_to_vec;
use crate::input_state::InputState;
use crate::sprite::Sprite;
use crate::sprite_builder::create_sprite;
use crate::sprite_type::SpriteType;
use crate::MainState;
use ggez::GameResult;

type Vector2 = glam::Vec2;

fn boost_speed(sprite: &mut Sprite, delta_time: f32) {
    let direction_vector = angle_to_vec(sprite.facing);
    let push_vector = Vector2 {
        x: direction_vector.x * PLAYER_PUSH_VALUE,
        y: direction_vector.y * PLAYER_PUSH_VALUE,
    };
    sprite.velocity.x = push_vector.x * delta_time;
    sprite.velocity.y = push_vector.y * delta_time;
}

pub fn handle_input(sprite: &mut Sprite, input: &InputState, delta_time: f32) {
    sprite.facing += delta_time * PLAYER_TURN_RATE * input.x_axis;
    if input.y_axis > 0. {
        boost_speed(sprite, delta_time);
    }
}

pub fn fire_at_will(main_state: &mut MainState) -> GameResult {
    main_state.player_shot_timeout = PLAYER_SHOT_TIME;

    let player = &main_state.player;
    let mut shot = create_sprite(SpriteType::Shot);
    shot.position.x = player.position.x;
    shot.position.y = player.position.y;
    shot.facing = player.facing;

    let direction = angle_to_vec(shot.facing);
    shot.velocity.x = SHOT_SPEED * direction.x;
    shot.velocity.y = SHOT_SPEED * direction.y;

    &main_state.shots.push(shot);

    Ok(())
}
