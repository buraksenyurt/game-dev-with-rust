use crate::common::constants::MAX_AMMO;
use crate::entity::asset::Asset;
use crate::entity::bullet::Bullet;
use crate::entity::fighter::Fighter;
use crate::entity::fleet::Fleet;
use crate::game::state::State;

pub struct Game {
    pub state: State,
    pub bullets: Vec<Bullet>,
    pub enemy_bullets: Vec<Bullet>,
    pub enemy_fleet: Fleet,
    pub fighter: Fighter,
    pub clouds: Vec<Asset>,
    pub extra_ammo: Option<Asset>,
}

impl Game {
    pub async fn new(state: State) -> Self {
        Self {
            state,
            bullets: Vec::default(),
            enemy_bullets: Vec::default(),
            enemy_fleet: Fleet::default(),
            fighter: Fighter::new().await,
            clouds: Vec::default(),
            extra_ammo: None,
        }
    }
}
