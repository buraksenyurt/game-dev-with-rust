use crate::game::game::Game;
use macroquad::prelude::Rect;

//Axis-Aligned Bounding Box (AABB) Collision Detection
async fn aabb_check(source: Rect, target: Rect) -> bool {
    if source.x < target.x + target.w
        && source.x + source.w > target.x
        && source.y < target.y + target.h
        && source.y + source.h > target.y
    {
        return true;
    }
    false
}
pub async fn check_fighter_with_ammo(game: &mut Game) -> bool {
    match game.extra_ammo_box {
        Some(ea) => {
            let fighter_box: Rect = game.fighter.get_body();
            let ammo_box = Rect::new(
                ea.location.x,
                ea.location.y,
                ea.texture.width(),
                ea.texture.height(),
            );
            aabb_check(ammo_box, fighter_box).await
        }
        None => false,
    }
}

pub async fn check_enmy_f_coll(game: &mut Game) {
    let body = game.fighter.get_body();
    let wing = game.fighter.get_wing();
    let tail_wing = game.fighter.get_tail_wing();
    for b in game.enemy_fighters.bullets.iter_mut() {
        let b_rect = b.get_rect().await;
        if aabb_check(b_rect, body).await
            || aabb_check(b_rect, wing).await
            || aabb_check(b_rect, tail_wing).await
        {
            b.is_alive = false;
            game.score_box.enemy_fighter_damage += 1;
        }
    }
}

pub async fn check_enmy_b_coll(game: &mut Game) {
    let body = game.fighter.get_body();
    let wing = game.fighter.get_wing();
    let tail_wing = game.fighter.get_tail_wing();
    for b in game.enemy_bombers.bullets.iter_mut() {
        let b_rect = b.get_rect().await;
        if aabb_check(b_rect, body).await
            || aabb_check(b_rect, wing).await
            || aabb_check(b_rect, tail_wing).await
        {
            b.is_alive = false;
            game.score_box.enemy_bomber_damage += 1;
        }
    }
}

pub async fn check_enmy_ws_coll(game: &mut Game) {
    let body = game.fighter.get_body();
    let wing = game.fighter.get_wing();
    let tail_wing = game.fighter.get_tail_wing();
    for b in game.enemy_warships.bullets.iter_mut() {
        let b_rect = b.get_rect().await;
        if aabb_check(b_rect, body).await
            || aabb_check(b_rect, wing).await
            || aabb_check(b_rect, tail_wing).await
        {
            b.is_alive = false;
            game.score_box.enemy_warship_damage += 1;
        }
    }
}
