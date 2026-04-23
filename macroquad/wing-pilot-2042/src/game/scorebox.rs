use std::fmt::{Display, Formatter};

#[derive(Default)]
pub struct Scorebox {
    pub player_hit: usize,
    pub enemy_fighter_damage: usize,
    pub enemy_bomber_damage: usize,
    pub enemy_warship_damage: usize,
}

impl Display for Scorebox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Player Hit:{}, Total Damage:{}",
            self.player_hit,
            self.enemy_bomber_damage + self.enemy_warship_damage + self.enemy_fighter_damage
        )
    }
}
