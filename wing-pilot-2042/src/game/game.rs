use crate::entity::asset::Asset;
use crate::entity::fighter::Fighter;
use crate::entity::fleet::Fleet;
use crate::game::state::State;

pub struct Game {
    pub state: State,
    pub enemy_fighters: Fleet,
    pub enemy_bombers: Fleet,
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
            fighter: Fighter::new().await,
            clouds: Vec::default(),
            extra_ammo: None,
        }
    }
}
