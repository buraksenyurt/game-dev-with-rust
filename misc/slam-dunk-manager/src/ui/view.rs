use crate::data::model::*;
use crate::prelude::colors::*;
use crate::prelude::utility::pause;
use clearscreen::clear;
use std::cmp::Ordering::Equal;

pub fn print_transfer_market(market: &TransferMarket) {
    println!("{light_yellow}{}{color_reset}", "-".repeat(64));
    println!(
        "{light_yellow}{} {color_blue}TRANSFER MARKET ({:.2}) Million ${light_yellow}{}{color_reset}",
        "-".repeat(15),
        market.total_value,
        "-".repeat(15)
    );
    println!("{light_yellow}{}{color_reset}", "-".repeat(64));
    for p in market.players.iter() {
        println!("{color_cyan}{}{color_reset}", p);
        println!("{color_magenta}{}{color_reset}", p.stats);
    }
    println!("{light_yellow}{}{color_reset}", "-".repeat(64));
}

pub fn print_table(league: &mut League) {
    league.teams.sort_by(|a, b| {
        let win_cmp = b.stats.win.cmp(&a.stats.win);
        if win_cmp == Equal {
            b.stats.diff.cmp(&a.stats.diff)
        } else {
            win_cmp
        }
    });
    println!("{light_yellow}{}", "-".repeat(64));
    println!("{:<24}", league.name);
    println!("{}{color_reset}", "-".repeat(64));
    println!(
        "{color_blue} {:<2}   {:<20} {:<3} {:<3} {:<3} {:<8} {:<8} {:<9}{color_reset}",
        " ", "Team Name", "GP", "W", "L", "Points+", "Points-", "Diff"
    );
    let mut i = 1;
    for team in league.teams.iter() {
        println!(
            "{color_cyan} {:<2} - {:<20} {:<3} {:<3} {:<3} {:<8} {:<8} {:<9}{color_reset}",
            i,
            team.name,
            team.stats.game_played,
            team.stats.win,
            team.stats.loss,
            team.stats.points_plus,
            team.stats.points_minus,
            team.stats.diff
        );
        i += 1;
    }
    println!("{light_yellow}{}{color_reset}", "-".repeat(64));
}

pub fn print_main_menu() {
    println!("{light_yellow}{}{color_reset}", "-".repeat(64));
    println!(
        "{light_yellow}{} {color_cyan}(N)ew Game {light_yellow}{}{color_reset}",
        "-".repeat(24),
        "-".repeat(28)
    );
    println!(
        "{light_yellow}{} {color_cyan}(L)oad Season {light_yellow}{}{color_reset}",
        "-".repeat(24),
        "-".repeat(25)
    );
    println!(
        "{light_yellow}{} {color_cyan}(T)ransfer Market {light_yellow}{}{color_reset}",
        "-".repeat(24),
        "-".repeat(21)
    );
    println!(
        "{light_yellow}{} {color_cyan}e(X)it Game {light_yellow}{}{color_reset}",
        "-".repeat(24),
        "-".repeat(27)
    );
    println!("{light_yellow}{}{color_reset}", "-".repeat(64));
    println!(
        "{color_magenta}Press N,L,T,X etc to Continue{}{color_reset}",
        " ".repeat(35)
    );
}

pub fn print_coach_team(team: &Team) {
    println!("{light_yellow}{}{color_reset}", "-".repeat(64));
    println!(
        "{light_yellow}{} {color_blue}{:<24}{light_yellow}{}{color_reset}",
        "-".repeat(20),
        team.name,
        "-".repeat(19)
    );
    println!(
        "{light_yellow}{} {color_blue}Attack Power {:.2} Defense Power {:.2}{light_yellow}{}{color_reset}",
        "-".repeat(12),
        team.attack_power,
        team.defensive_power,
        "-".repeat(15)
    );
    println!("{light_yellow}{}{color_reset}", "-".repeat(64));
    for p in team.players.iter() {
        println!("{color_cyan}{}{color_reset}", p);
        println!("{color_magenta}{}{color_reset}", p.stats);
    }
    println!("{light_yellow}{}{color_reset}", "-".repeat(64));
}

pub fn print_fixture_by_paging(match_days: &[MatchDay]) {
    for (idx, md) in match_days.iter().enumerate() {
        clear().unwrap();
        print_match_day(idx, md);
        pause("For the next match day press any key to show.");
    }
}

pub fn print_fixture(match_days: &[MatchDay]) {
    clear().unwrap();
    for (idx, md) in match_days.iter().enumerate() {
        print_match_day(idx, md);
    }
}

fn print_match_day(idx: usize, md: &MatchDay) {
    println!("{light_yellow}{}{color_reset}", "-".repeat(64));
    println!(
        "{light_yellow}{} {color_blue}League Fixture{light_yellow} {}{color_reset}",
        "-".repeat(24),
        "-".repeat(24)
    );
    println!(
        "{light_yellow}{} {color_blue}Match Day - {}{light_yellow} {}{color_reset}",
        "-".repeat(24),
        idx + 1,
        "-".repeat(25)
    );
    for m in md.competitions.iter() {
        println!("\t{color_magenta}{}{color_reset}", m);
    }
}
