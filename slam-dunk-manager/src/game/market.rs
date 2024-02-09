use crate::data::model::TransferMarket;
use crate::prelude::model::Player;

pub fn get_player(player_number: u16, transfer_market: &TransferMarket) -> Option<&Player> {
    if let Some(p) = transfer_market
        .players
        .iter()
        .find(|p| p.id == player_number)
    {
        p;
    }
    None
}
