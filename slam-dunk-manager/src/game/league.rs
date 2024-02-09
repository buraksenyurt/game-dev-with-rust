use crate::data::teams::generate_teams;
use crate::prelude::model::*;
use rand::{thread_rng, Rng};
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

pub fn add_player_team(name: String, league: &mut League) -> Team {
    let mut rng = thread_rng();
    let player_team = Team {
        name,
        players: vec![],
        attack_power: rng.gen_range(5.0..10.0),
        defensive_power: rng.gen_range(5.0..10.0),
        stats: Default::default(),
    };
    league.teams.push(player_team.clone());
    player_team.clone()
}
