use crate::entity::enemy_type::EnemyType;
use crate::entity::fleet::Fleet;
use crate::game::game::Game;
use macroquad::prelude::Rect;
use crate::entity::bullet::Bullet;

//Axis-Aligned Bounding Box (AABB) Collision Detection
async fn aabb_check(source: Rect, target: &Rect) -> bool {
    if source.x < target.x + target.w
        && source.x + source.w > target.x
        && source.y < target.y + target.h
        && source.y + source.h > target.y
    {
        return true;
    }
    false
}

async fn get_fighter_bodies(game: &mut Game) -> Vec<Rect> {
    let body = game.fighter.get_body();
    let wing = game.fighter.get_wing();
    let tail_wing = game.fighter.get_tail_wing();
    vec![body, wing, tail_wing]
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
            aabb_check(ammo_box, &fighter_box).await
        }
        None => false,
    }
}

pub async fn check_enemy_f_coll(game: &mut Game) {
    if exist_collision(get_fighter_bodies(game).await, &mut game.enemy_fighters).await {
        game.score_box.enemy_fighter_damage += 3;
        game.fighter.shield -= 3;
        game.fighter.is_got_shot = true;
        game.fighter.shot_owner = EnemyType::Fighter;
    }
}

pub async fn check_enemy_b_coll(game: &mut Game) {
    if exist_collision(get_fighter_bodies(game).await, &mut game.enemy_bombers).await {
        game.score_box.enemy_bomber_damage += 2;
        game.fighter.shield -= 2;
        game.fighter.is_got_shot = true;
        game.fighter.shot_owner = EnemyType::Bomber;
    }
}

pub async fn check_enemy_ws_coll(game: &mut Game) {
    if exist_collision(get_fighter_bodies(game).await, &mut game.enemy_warships).await {
        game.score_box.enemy_warship_damage += 1;
        game.fighter.shield -= 1;
        game.fighter.is_got_shot = true;
        game.fighter.shot_owner = EnemyType::Warship(None);
    }
}

pub async fn exist_collision(rect_list: Vec<Rect>, fleet: &mut Fleet) -> bool {
    for b in fleet.bullets.iter_mut() {
        let b_rect = b.get_rect().await;
        for r in rect_list.iter() {
            if aabb_check(b_rect, r).await {
                b.is_alive=false;
                return true;
            }
        }
    }
    false
}
