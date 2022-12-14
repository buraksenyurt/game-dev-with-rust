use crate::common::constants::{
    ENEMY_BOMBER_SPEED_FACTOR, ENEMY_FIGHTER_SPEED_FACTOR, ENEMY_WARSHIP_SPEED_FACTOR,
};
use crate::entity::asset::Asset;
use crate::entity::enemy_type::EnemyType;
use crate::entity::fighter::Fighter;
use crate::entity::fleet::Fleet;
use crate::game::state::State;

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
}
