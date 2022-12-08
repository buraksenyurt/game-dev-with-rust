use crate::common::constants::MAX_AMMO;
use crate::entity::asset::Asset;
use crate::entity::bullet::Bullet;
use crate::entity::fleet::Fleet;
use crate::game::state::State;
use macroquad::ui::Drag::No;

pub struct Game {
    pub state: State,
    pub bullets: Vec<Bullet>,
    pub enemy_fleet: Fleet,
    pub fighter_amount_count: usize,
    pub clouds: Vec<Asset>,
    pub extra_ammo: Option<Asset>,
}

impl Game {
    pub fn new(state: State) -> Self {
        Self {
            state,
            bullets: Vec::default(),
            enemy_fleet: Fleet::default(),
            fighter_amount_count: MAX_AMMO,
            clouds: Vec::default(),
            extra_ammo: None,
        }
    }
}
