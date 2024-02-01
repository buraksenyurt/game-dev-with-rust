use crate::data::constants::{
    DEFAULT_TEAM_ATTACK_POWER, DEFAULT_TEAM_DEFENSE_POWER, MAX_PLAYER_COUNT,
};
use crate::data::model::*;
use crate::data::players::*;

pub fn create_teams() -> Vec<Team> {
    let mut all_players = generate_players();
    let mut teams = Vec::new();

    let team_names = [
        "Eagles", "Wolves", "Sharks", "Dragons", "Lions", "Bears", "Hawks", "Panthers",
    ];

    for name in team_names.iter() {
        if all_players.len() < MAX_PLAYER_COUNT {
            break;
        }

        let team_players = all_players.drain(0..MAX_PLAYER_COUNT).collect();

        teams.push(Team {
            name: name.to_string(),
            players: team_players,
            attack_power: DEFAULT_TEAM_ATTACK_POWER,
            defensive_power: DEFAULT_TEAM_DEFENSE_POWER,
        });
    }

    teams
}
