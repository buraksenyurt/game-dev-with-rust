use crate::entity::bullet::Bullet;
use crate::entity::bullet_type::BulletType;
use crate::entity::enemy_type::EnemyType;
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
    for b in game.fighter.bullets.iter_mut() {
        for f in game.enemy_fighters.actors.iter_mut() {
            let bodies = vec![
                f.get_body().await.unwrap(),
                f.get_wing().await.unwrap(),
                f.get_tail_wing().await,
            ];
            if is_collision_exist(bodies, b).await {
                f.shield -= 1;
                f.is_got_shot = true;
                if f.shield <= 0 {
                    f.on_stage = false;
                    game.wining_criteria.max_fighter -= 1;
                    game.score_box.player_hit += 1;
                }
            }
        }
    }
    for b in game.enemy_fighters.bullets.iter_mut() {
        let bodies = vec![
            game.fighter.get_body().await,
            game.fighter.get_wing().await,
            game.fighter.get_tail_wing().await,
        ];
        if is_collision_exist(bodies, b).await {
            game.score_box.enemy_fighter_damage += 3;
            game.fighter.shield -= 3;
            game.fighter.is_got_shot = true;
            game.fighter.shot_owner = EnemyType::Fighter;
        }
    }
}

pub async fn fighter_vs_bomber(game: &mut Game) {
    for b in game.fighter.bullets.iter_mut() {
        for bmbr in game.enemy_bombers.actors.iter_mut() {
            let bodies = vec![
                bmbr.get_body().await.unwrap(),
                bmbr.get_wing().await.unwrap(),
                bmbr.get_tail_wing().await,
            ];
            if is_collision_exist(bodies, b).await {
                bmbr.shield -= 1;
                bmbr.is_got_shot = true;
                if bmbr.shield <= 0 {
                    bmbr.on_stage = false;
                    game.wining_criteria.max_bomber -= 1;
                    game.score_box.player_hit += 1;
                }
            }
        }
    }
    for b in game.enemy_bombers.bullets.iter_mut() {
        let bodies = vec![
            game.fighter.get_body().await,
            game.fighter.get_wing().await,
            game.fighter.get_tail_wing().await,
        ];
        if is_collision_exist(bodies, b).await {
            game.score_box.enemy_bomber_damage += 2;
            game.fighter.shield -= 2;
            game.fighter.is_got_shot = true;
            game.fighter.shot_owner = EnemyType::Bomber;
        }
    }
}

pub async fn fighter_vs_warship(game: &mut Game) {
    for b in game.fighter.bullets.iter_mut() {
        for ws in game.enemy_warships.actors.iter_mut() {
            let bodies = vec![ws.get_body().await.unwrap()];
            if is_collision_exist(bodies, b).await {
                ws.shield -= 1;
                ws.is_got_shot = true;
                if ws.shield <= 0 {
                    ws.on_stage = false;
                    game.wining_criteria.max_warship -= 1;
                    game.score_box.player_hit += 1;
                    game.enemy_warships
                        .bullets
                        .iter_mut()
                        .for_each(|mut b| b.is_alive = false);
                }
            }
        }
    }
    for b in game.enemy_warships.bullets.iter_mut() {
        let bodies = vec![
            game.fighter.get_body().await,
            game.fighter.get_wing().await,
            game.fighter.get_tail_wing().await,
        ];
        if is_collision_exist(bodies, b).await {
            game.score_box.enemy_warship_damage += 1;
            game.fighter.shield -= 1;
            game.fighter.is_got_shot = true;
            game.fighter.shot_owner = EnemyType::Warship(None);
        }
    }
}

pub async fn fighter_vs_warship_missile(game: &mut Game) {
    for m in game.enemy_warships.bullets.iter_mut() {
        let rect = Rect::new(m.location.x - 32., m.location.y - 32., 64., 64.);
        if let Some(b) = game
            .fighter
            .bullets
            .iter_mut()
            .find(|b| b.bullet_type == BulletType::ContraMissile)
        {
            //let v = Vec2::new(b.location.x - m.location.x, b.location.y - m.location.y);
            if is_collision_exist(vec![rect], b).await {
                b.is_alive = false;
                m.is_alive = false;
            }
        }
    }
}

pub async fn warship_vs_ground(game: &mut Game) {
    for ws in game.enemy_warships.actors.iter_mut() {
        for g in game.grounds.iter() {
            let ws_body = ws.get_body().await.unwrap();
            let g_body = g.get_rect(0.2).await;
            if aabb_check(ws_body, &g_body).await {
                ws.wait = true;
            } else {
                ws.wait = false;
            }
        }
    }
}

pub async fn is_collision_exist(rect_list: Vec<Rect>, bullet: &mut Bullet) -> bool {
    let b_rect = bullet.get_rect().await;
    for r in rect_list.iter() {
        if aabb_check(b_rect, r).await {
            bullet.is_alive = false;
            return true;
        }
    }
    false
}
