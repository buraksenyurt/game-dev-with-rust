use crate::data::teams::generate_teams;
use crate::prelude::model::*;
use std::time::SystemTime;

pub fn create_league(name: String) -> League {
    let (teams, available_players) = generate_teams();
    let total_market_value = &available_players.iter().map(|p| p.transfer_fee).sum();
    let transfer_market = TransferMarket {
        players: available_players,
        total_value: *total_market_value,
    };
    League {
        name,
        teams,
        start_date: SystemTime::now().into(),
        transfer_market,
        is_active: false,
    }
}
