use crate::data::teams::generate_teams;
use crate::game_mngr::market::take_player;
use crate::prelude::colors::*;
use crate::prelude::model::*;
use crate::prelude::utility::get_input;
use rand::{thread_rng, Rng};
use std::time::SystemTime;

pub fn create_league() -> League {
    let name = get_league_name();
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

pub fn add_player_team(name: &str, league: &mut League) -> Team {
    let mut rng = thread_rng();
    let player_team = Team {
        name: name.to_string(),
        players: vec![],
        attack_power: rng.gen_range(5.0..10.0),
        defensive_power: rng.gen_range(5.0..10.0),
        stats: Default::default(),
    };
    league.teams.push(player_team.clone());
    player_team.clone()
}

pub fn add_players_to_team(league: &mut League, player_team: &mut Team) {
    let mut player_count = 0;
    while player_count <= 4 {
        println!("{color_magenta}Please enter player's number. Be careful!{color_reset}");
        if let Ok(n) = get_input().unwrap().parse::<u16>() {
            if player_team.players.iter().any(|p| p.id == n) {
                println!("{color_red}This player has already been added your team.{color_reset}",);
                continue;
            }
            if let Some(mut p) = take_player(n, &league.transfer_market.players) {
                p.is_free = false;
                println!(
                    "{color_green}{} has been added your team.{color_reset}",
                    &p.full_name
                );
                player_team.players.push(p);
                player_count += 1;
            } else {
                println!("{color_red}Player not found in transfer market!{color_reset}");
                continue;
            }
        } else {
            println!("{color_red}Please enter a valid number!{color_reset}");
            continue;
        }
    }
}

fn get_league_name() -> String {
    let mut rng = thread_rng();
    let possibilities = vec![
        "MVP Champions League",
        "Red Dragons Guild",
        "Naruto San Masters",
        "Emerald Knights League",
        "Silver Phoenix Syndicate",
        "Golden Griffins Circuit",
        "Crystal Unicorn Alliance",
        "Black Panthers Domain",
        "Mystic Tigers Federation",
        "Thunder Wolves Division",
        "Scarlet Samurai Arena",
        "Azure Dragons Coalition",
        "Neon Ninjas Brotherhood",
        "Platinum Pirates Assembly",
        "Crimson Wizards Society",
        "Sapphire Eagles Conglomerate",
        "Violet Valkyries League",
        "Shadow Lynx Legion",
        "Firestorm Phoenix Crew",
        "Galactic Gladiators League",
    ];
    let index = rng.gen_range(0..possibilities.len());
    possibilities[index].to_string()
}
