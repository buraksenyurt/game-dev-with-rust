use crate::common::constants::{
    CLOUD_SPEED_FACTOR, ENEMY_BOMBER_SPEED_FACTOR, ENEMY_FIGHTER_SPEED_FACTOR,
    ENEMY_WARSHIP_SPEED_FACTOR, FIGHTER_BULLET_SPEED_FACTOR, INFO_BAR_MARGIN,
};
use crate::entity::asset::Asset;
use crate::entity::enemy_type::{EnemyType, WarshipDirection};
use crate::entity::fighter::Fighter;
use crate::entity::fleet::Fleet;
use crate::game::scorebox::Scorebox;
use crate::game::state::State;
use macroquad::prelude::{draw_text, measure_text, screen_height, Vec2, GOLD};
use macroquad::time::get_frame_time;
use macroquad::window::screen_width;
use std::f32::consts::PI;

pub struct Game {
    pub state: State,
    pub enemy_fighters: Fleet,
    pub enemy_bombers: Fleet,
    pub enemy_warships: Fleet,
    pub fighter: Fighter,
    pub clouds: Vec<Asset>,
    pub extra_ammo_box: Option<Asset>,
    pub score_box: Scorebox,
}

impl Game {
    pub async fn new(state: State) -> Self {
        Self {
            state,
            enemy_fighters: Fleet::default(),
            enemy_bombers: Fleet::default(),
            enemy_warships: Fleet::default(),
            fighter: Fighter::new().await,
            clouds: Vec::default(),
            extra_ammo_box: None,
            score_box: Scorebox::default(),
        }
    }

    pub async fn draw_fleet(&mut self, actor: EnemyType) {
        let (enemies, speed_factor) = match actor {
            EnemyType::Fighter => (&mut self.enemy_fighters.actors, ENEMY_FIGHTER_SPEED_FACTOR),
            EnemyType::Bomber => (&mut self.enemy_bombers.actors, ENEMY_BOMBER_SPEED_FACTOR),
            EnemyType::Warship(_) => (&mut self.enemy_warships.actors, ENEMY_WARSHIP_SPEED_FACTOR),
        };

        for e in enemies.iter_mut() {
            e.position += e.velocity * speed_factor;
            match actor {
                EnemyType::Warship(_) => {
                    //info!("Warship position {}", e.position);
                    if e.position.x >= screen_width() * 0.3 || e.position.x < screen_width() * 0.7 {
                        //e.velocity = e.formation.velocity;
                        //e.is_formation_on = true;
                        e.fire_at_will = true;
                    }
                }
                _ => {
                    if !e.is_formation_on && e.position.y >= e.formation.start_y {
                        e.velocity = e.formation.velocity;
                        e.is_formation_on = true;
                        e.fire_at_will = true;
                    }
                }
            }

            e.check_borders().await;
            e.draw().await;
        }
    }

    pub async fn draw_bullets(&mut self, actor: EnemyType) {
        let (bullets, speed_factor) = match actor {
            EnemyType::Bomber => (&mut self.enemy_bombers.bullets, ENEMY_BOMBER_SPEED_FACTOR),
            EnemyType::Fighter => (&mut self.enemy_fighters.bullets, ENEMY_FIGHTER_SPEED_FACTOR),
            EnemyType::Warship(_) => (&mut self.enemy_warships.bullets, ENEMY_BOMBER_SPEED_FACTOR),
        };
        for b in bullets.iter_mut() {
            b.location += b.velocity * speed_factor;
            b.draw().await;
            if b.location.y > screen_height() {
                b.is_alive = false;
            }
        }
    }

    pub async fn draw_fighter_bullets(&mut self) {
        for b in self.fighter.bullets.iter_mut() {
            b.location += Vec2::new(0., -1.) * FIGHTER_BULLET_SPEED_FACTOR;
            b.draw().await;
            if b.location.x < 0. {
                b.is_alive = false;
            }
        }
    }

    pub async fn draw_clouds(&mut self) {
        for c in self.clouds.iter_mut() {
            c.location += c.velocity * CLOUD_SPEED_FACTOR;
            if c.location.y - c.texture.height() > screen_height() {
                c.on_stage = false;
            }
            c.draw();
        }
    }

    pub async fn draw_info_bar(&self) {
        let info = format!(
            "Bullets {} (F:{}) (B:{}) (WS:{}) Damages (F/B/WS) {}/{}/{} Player Hit/Damage {}/{}",
            self.fighter.ammo_count,
            self.enemy_fighters.actors.len(),
            self.enemy_bombers.actors.len(),
            self.enemy_warships.actors.len(),
            self.score_box.enemy_fighter_damage,
            self.score_box.enemy_bomber_damage,
            self.score_box.enemy_warship_damage,
            self.score_box.player_hit,
            self.fighter.shield
        );
        let size = measure_text(info.as_str(), None, 24, 1.);
        draw_text(
            info.as_str(),
            INFO_BAR_MARGIN,
            size.height + INFO_BAR_MARGIN,
            24.,
            GOLD,
        );
    }

    pub async fn enemy_shot(&mut self) {
        for enemy in self.enemy_fighters.actors.iter_mut() {
            if enemy.fire_at_will {
                let bullets = enemy.spawn_bullets(Vec2::new(0., 1.), 0.).await;
                if let Some(mut b) = bullets {
                    self.enemy_fighters.bullets.append(&mut b);
                }
            }
        }
    }

    pub async fn bomber_shot(&mut self) {
        for enemy in self.enemy_bombers.actors.iter_mut() {
            if enemy.fire_at_will {
                let v = (self.fighter.get_muzzle_point().await - enemy.get_muzzle_point().await)
                    .normalize();
                let angle = 2. * PI - v.angle_between(Vec2::new(1., 0.));
                let vel = Vec2::new(angle.cos(), angle.sin());
                let bullets = enemy.spawn_bullets(vel, angle).await;
                if let Some(mut b) = bullets {
                    self.enemy_bombers.bullets.append(&mut b);
                }
            }
        }
    }

    pub async fn warship_shot(&mut self) {
        for enemy in self.enemy_warships.actors.iter_mut() {
            if enemy.fire_at_will {
                let v = (self.fighter.get_muzzle_point().await - enemy.get_muzzle_point().await)
                    .normalize();
                let angle = 2. * PI - v.angle_between(Vec2::new(1., 0.));
                let vel = Vec2::new(angle.cos(), angle.sin());
                let bullets = enemy.spawn_bullets(vel, angle).await;
                if let Some(mut b) = bullets {
                    self.enemy_warships.bullets.append(&mut b);
                }
            }
        }
    }

    pub async fn recalc_distance(&mut self) {
        for b in self.enemy_warships.bullets.iter_mut() {
            let v = (self.fighter.get_muzzle_point().await - b.location).normalize();
            let angle = 2. * PI - v.angle_between(Vec2::new(1., 0.));
            let vel = Vec2::new(angle.cos(), angle.sin());
            b.rotation = angle;
            b.velocity = vel;
        }
    }

    pub async fn spawn_enemy_fighters(&mut self) {
        if self.enemy_fighters.actors.is_empty() && self.enemy_fighters.bullets.is_empty() {
            if self.enemy_fighters.lift_off_time == 0 {
                self.enemy_fighters = Fleet::new(4, EnemyType::Fighter).await;
            } else {
                self.enemy_fighters.lift_off_time -= 1;
            }
        }
    }

    pub async fn spawn_enemy_warships(&mut self) {
        if self.enemy_warships.actors.is_empty() && self.enemy_warships.bullets.is_empty() {
            let v = get_frame_time().floor() % 2.;
            let warship_direction = match v == 0. {
                true => Some(WarshipDirection::Left),
                _ => Some(WarshipDirection::Right),
            };
            //println!("Left or right {}", v);
            if self.enemy_warships.lift_off_time == 0 {
                self.enemy_warships = Fleet::new(1, EnemyType::Warship(warship_direction)).await;
            } else {
                self.enemy_warships.lift_off_time -= 1;
            }
        }
    }

    pub async fn spawn_enemy_bombers(&mut self) {
        if self.enemy_bombers.actors.is_empty() && self.enemy_bombers.bullets.is_empty() {
            if self.enemy_bombers.lift_off_time == 0 {
                self.enemy_bombers = Fleet::new(3, EnemyType::Bomber).await;
            } else {
                self.enemy_bombers.lift_off_time -= 1;
            }
        }
    }
}
