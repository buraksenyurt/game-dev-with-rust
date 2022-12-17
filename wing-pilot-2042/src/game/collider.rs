use crate::game::game::Game;
use macroquad::prelude::Rect;

pub fn check_fighter_with_ammo(game: &mut Game) -> bool {
    //Axis-Aligned Bounding Box (AABB) Collision Detection
    match game.extra_ammo_box {
        Some(ea) => {
            let fighter_box: Rect = game.fighter.get_body();
            if ea.location.x < fighter_box.x + fighter_box.w
                && ea.location.x + ea.texture.width() > fighter_box.x
                && ea.location.y < fighter_box.y + fighter_box.h
                && ea.location.y + ea.texture.height() > fighter_box.y
            {
                return true;
            }
            false
        }
        None => false,
    }
}
