use crate::data::constants::*;
use crate::data::model::*;
use rand::{Rng, thread_rng};
pub fn create_teams(players: &mut Vec<Player>) -> Vec<Team> {
    let mut rng = thread_rng();
    let mut teams = Vec::new();

    let team_names = vec!["Eagles", "Wolves", "Sharks", "Dragons", "Lions", "Bears", "Hawks"];

    for name in team_names.iter() {
        let positions_required = vec![Position::Guard, Position::PowerForward, Position::Center];
        let mut team_players = Vec::new();

        for &position in &positions_required {
            if let Some(index) = players.iter().position(|p| p.position == position) {
                team_players.push(players.remove(index));
            } else {
                return teams;
            }
        }

        while team_players.len() < MAX_PLAYER_COUNT {
            let index = rng.gen_range(0..players.len());
            team_players.push(players.remove(index));
        }

        teams.push(Team {
            name: name.to_string(),
            players: team_players,
            attack_power: DEFAULT_TEAM_ATTACK_POWER,
            defensive_power: DEFAULT_TEAM_DEFENSE_POWER,
        });
    }

    teams
}