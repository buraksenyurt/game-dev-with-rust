use crate::constant::{PLAYER_PUSH_VALUE, PLAYER_TURN_RATE};
use crate::fermat::angle_to_vec;
use crate::input_state::InputState;
use crate::sprite::Sprite;
type Vector2 = glam::Vec2;

fn push(sprite: &mut Sprite, delta_time: f32) {
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
        push(sprite, delta_time);
    }
}
