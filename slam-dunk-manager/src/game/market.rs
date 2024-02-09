use crate::prelude::model::Player;
pub fn get_player(player_number: u16, players: &Vec<Player>) -> Option<Player> {
    if let Some(p) = players.iter().find(|p| p.id == player_number) {
        return Some(p.clone());
    }
    None
}
