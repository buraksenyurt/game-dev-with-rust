use crate::common::constants::{
    CLOUD_SPEED_FACTOR, ENEMY_BOMBER_SPEED_FACTOR, ENEMY_FIGHTER_SPEED_FACTOR,
    ENEMY_WARSHIP_SPEED_FACTOR, FIGHTER_BULLET_SPEED_FACTOR, INFO_BAR_MARGIN,
};
use crate::entity::asset::Asset;
use crate::entity::enemy_type::EnemyType;
use crate::entity::fighter::Fighter;
use crate::entity::fleet::Fleet;
use crate::game::state::State;
use macroquad::prelude::{draw_text, measure_text, screen_height, Vec2, GOLD};

pub struct Game {
    pub state: State,
    pub enemy_fighters: Fleet,
    pub enemy_bombers: Fleet,
    pub enemy_warships: Fleet,
    pub fighter: Fighter,
    pub clouds: Vec<Asset>,
    pub extra_ammo: Option<Asset>,
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
            extra_ammo: None,
        }
    }

    pub async fn draw_fleet(&mut self, actor: EnemyType) {
        let (enemies, speed_factor) = match actor {
            EnemyType::Fighter => (&mut self.enemy_fighters.actors, ENEMY_FIGHTER_SPEED_FACTOR),
            EnemyType::Bomber => (&mut self.enemy_bombers.actors, ENEMY_BOMBER_SPEED_FACTOR),
            EnemyType::Warship => (&mut self.enemy_warships.actors, ENEMY_WARSHIP_SPEED_FACTOR),
        };

        for e in enemies.iter_mut() {
            e.position += e.velocity * speed_factor;
            if !e.is_formation_on && e.position.y >= e.formation.start_y {
                e.velocity = e.formation.velocity;
                e.is_formation_on = true;
                e.fire_at_will = true;
            }

            e.check_borders().await;
            e.draw().await;
        }
    }

    pub async fn draw_bullets(&mut self, actor: EnemyType) {
        let (bullets, speed_factor) = match actor {
            EnemyType::Bomber => (&mut self.enemy_bombers.bullets, ENEMY_BOMBER_SPEED_FACTOR),
            EnemyType::Fighter => (&mut self.enemy_fighters.bullets, ENEMY_FIGHTER_SPEED_FACTOR),
            EnemyType::Warship => (&mut self.enemy_warships.bullets, ENEMY_BOMBER_SPEED_FACTOR),
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
            "Bullets {} Enemies (F:{}) (B:{})",
            self.fighter.ammo_count,
            self.enemy_fighters.actors.len(),
            self.enemy_bombers.actors.len(),
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
}
