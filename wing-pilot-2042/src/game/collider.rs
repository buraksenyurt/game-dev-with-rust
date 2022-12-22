use crate::entity::bullet::Bullet;
use crate::entity::enemy_type::{EnemyType, WarshipDirection};
use crate::game::game::Game;
use macroquad::prelude::Rect;

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
    let body = game.fighter.get_body().await;
    let wing = game.fighter.get_wing().await;
    let tail_wing = game.fighter.get_tail_wing().await;
    vec![body, wing, tail_wing]
}

async fn get_enemy_bodies(game: &mut Game, et: EnemyType) -> Vec<Rect> {
    let mut sources = Vec::new();
    match et {
        EnemyType::Fighter => {
            for ef in game.enemy_fighters.actors.iter() {
                sources.push(ef.get_body().await.unwrap());
                sources.push(ef.get_wing().await.unwrap());
                sources.push(ef.get_tail_wing().await);
            }
        }
        EnemyType::Bomber => {
            for eb in game.enemy_bombers.actors.iter() {
                sources.push(eb.get_body().await.unwrap());
                sources.push(eb.get_wing().await.unwrap());
                sources.push(eb.get_tail_wing().await);
            }
        }
        EnemyType::Warship(Some(_ws)) => {
            for ws in game.enemy_warships.actors.iter() {
                sources.push(ws.get_body().await.unwrap());
            }
        }
        _ => (),
    }
    sources
}

// async fn get_bomber_bodies(game: &mut Game) -> Vec<Rect> {
//
// }

pub async fn check_fighter_with_ammo(game: &mut Game) -> bool {
    match game.extra_ammo_box {
        Some(ea) => {
            let fighter_box: Rect = game.fighter.get_body().await;
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

pub async fn fighter_vs_fighter(game: &mut Game) {
    if is_collision_exist(
        get_fighter_bodies(game).await,
        &mut game.enemy_fighters.bullets,
    )
    .await
    {
        game.score_box.enemy_fighter_damage += 3;
        game.fighter.shield -= 3;
        game.fighter.is_got_shot = true;
        game.fighter.shot_owner = EnemyType::Fighter;
    } else if is_collision_exist(
        get_enemy_bodies(game, EnemyType::Fighter).await,
        &mut game.fighter.bullets,
    )
    .await
    {
        println!("EF - Hitted the enemy fighter");
        // game.score_box.enemy_fighter_damage += 3;
        // game.fighter.shield -= 3;
        // game.fighter.is_got_shot = true;
        // game.fighter.shot_owner = EnemyType::Fighter;
    }
}

pub async fn fighter_vs_bomber(game: &mut Game) {
    if is_collision_exist(
        get_fighter_bodies(game).await,
        &mut game.enemy_bombers.bullets,
    )
    .await
    {
        game.score_box.enemy_bomber_damage += 2;
        game.fighter.shield -= 2;
        game.fighter.is_got_shot = true;
        game.fighter.shot_owner = EnemyType::Bomber;
    } else if is_collision_exist(
        get_enemy_bodies(game, EnemyType::Bomber).await,
        &mut game.fighter.bullets,
    )
    .await
    {
        println!("B - Hitted the bomber");
        // game.score_box.enemy_fighter_damage += 3;
        // game.fighter.shield -= 3;
        // game.fighter.is_got_shot = true;
        // game.fighter.shot_owner = EnemyType::Fighter;
    }
}

pub async fn fighter_vs_warship(game: &mut Game) {
    if is_collision_exist(
        get_fighter_bodies(game).await,
        &mut game.enemy_warships.bullets,
    )
    .await
    {
        game.score_box.enemy_warship_damage += 1;
        game.fighter.shield -= 1;
        game.fighter.is_got_shot = true;
        game.fighter.shot_owner = EnemyType::Warship(None);
    } else if is_collision_exist(
        get_enemy_bodies(game, EnemyType::Warship(Some(WarshipDirection::Right))).await,
        &mut game.fighter.bullets,
    )
    .await
    {
        println!("WS - Hitted the enemy Warship");
        // game.score_box.enemy_fighter_damage += 3;
        // game.fighter.shield -= 3;
        // game.fighter.is_got_shot = true;
        // game.fighter.shot_owner = EnemyType::Fighter;
    }
}

pub async fn is_collision_exist(rect_list: Vec<Rect>, bullets: &mut [Bullet]) -> bool {
    for b in bullets.iter_mut() {
        let b_rect = b.get_rect().await;
        for r in rect_list.iter() {
            if aabb_check(b_rect, r).await {
                b.is_alive = false;
                return true;
            }
        }
    }
    false
}
