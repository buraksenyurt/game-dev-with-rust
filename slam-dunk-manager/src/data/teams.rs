use crate::data::constants::*;
use crate::data::model::*;
use crate::data::players::load_players;
use rand::{thread_rng, Rng};

pub fn generate_teams() -> (Vec<Team>, Vec<Player>) {
    let mut rng = thread_rng();
    let mut teams = Vec::new();
    let mut players = load_players();
    let team_names = vec![
        "Eagles", "Wolves", "Sharks", "Dragons", "Lions", "Bears", "Hawks",
    ];

    for name in team_names.iter() {
        let positions_required = vec![Position::Guard, Position::PowerForward, Position::Center];
        let mut team_players = Vec::new();

        for &position in &positions_required {
            if let Some(index) = players.iter().position(|p| p.position == position) {
                team_players.push(players.remove(index));
            }
        }

        while team_players.len() < MAX_PLAYER_COUNT {
            let index = rng.gen_range(0..players.len());
            team_players.push(players.remove(index));
        }

        teams.push(Team {
            name: name.to_string(),
            players: team_players,
            attack_power: rng.gen_range(5.0..10.0),
            defensive_power: rng.gen_range(5.0..10.0),
            stats: Default::default(),
        });
    }

    (teams, players)
}
