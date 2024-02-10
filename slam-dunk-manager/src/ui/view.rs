use crate::data::model::*;
use inline_colorization::*;
use std::cmp::Ordering::Equal;

pub fn print_transfer_market(market: &TransferMarket) {
    println!("{color_bright_yellow}{}{color_reset}", "-".repeat(64));
    println!(
        "{color_bright_yellow}{} {color_bright_blue}TRANSFER MARKET ({:.2}) Million ${color_bright_yellow}{}{color_reset}",
        "-".repeat(15),
        market.total_value,
        "-".repeat(15)
    );
    println!("{color_bright_yellow}{}{color_reset}", "-".repeat(64));
    for p in market.players.iter() {
        println!("{color_cyan}{}{color_reset}", p);
        println!("{color_bright_magenta}{}{color_reset}", p.stats);
    }
    println!("{color_bright_yellow}{}{color_reset}", "-".repeat(64));
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
    println!("{color_bright_yellow}{}", "-".repeat(64));
    println!("{:<24}", league.name);
    println!("{}{color_reset}", "-".repeat(64));
    println!(
        "{color_bright_blue} {:<2}   {:<20} {:<3} {:<3} {:<3} {:<8} {:<8} {:<9}{color_reset}",
        " ", "Team Name", "GP", "W", "L", "Points+", "Points-", "Diff"
    );
    let mut i = 1;
    for team in league.teams.iter() {
        println!(
            "{color_bright_cyan} {:<2} - {:<20} {:<3} {:<3} {:<3} {:<8} {:<8} {:<9}{color_reset}",
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
    println!("{color_bright_yellow}{}{color_reset}", "-".repeat(64));
}

pub fn print_main_menu() {
    println!("{color_yellow}{}{color_reset}", "-".repeat(64));
    println!(
        "{color_bright_yellow}{} {color_cyan}(N)ew Game {color_bright_yellow}{}{color_reset}",
        "-".repeat(24),
        "-".repeat(28)
    );
    println!(
        "{color_bright_yellow}{} {color_cyan}(L)oad Season {color_bright_yellow}{}{color_reset}",
        "-".repeat(24),
        "-".repeat(25)
    );
    println!(
        "{color_bright_yellow}{} {color_cyan}(T)ransfer Market {color_bright_yellow}{}{color_reset}",
        "-".repeat(24),
        "-".repeat(21)
    );
    println!(
        "{color_bright_yellow}{} {color_cyan}e(X)it Game {color_bright_yellow}{}{color_reset}",
        "-".repeat(24),
        "-".repeat(27)
    );
    println!("{color_yellow}{}{color_reset}", "-".repeat(64));
    println!(
        "{color_magenta}Press N,L,T,X etc to Continue{}{color_reset}",
        " ".repeat(35)
    );
}

pub fn print_coach_team(team: &Team) {
    println!("{color_yellow}{}{color_reset}", "-".repeat(64));
    println!(
        "{color_bright_yellow}{} {color_bright_blue}{:<24}{color_bright_yellow}{}{color_reset}",
        "-".repeat(20),
        team.name,
        "-".repeat(19)
    );
    println!(
        "{color_bright_yellow}{} {color_bright_blue}Attack Power {:.2} Defense Power {:.2}{color_bright_yellow}{}{color_reset}",
        "-".repeat(12),
        team.attack_power,
        team.defensive_power,
        "-".repeat(15)
    );
    println!("{color_bright_yellow}{}{color_reset}", "-".repeat(64));
    for p in team.players.iter() {
        println!("{color_cyan}{}{color_reset}", p);
        println!("{color_bright_magenta}{}{color_reset}", p.stats);
    }
    println!("{color_bright_yellow}{}{color_reset}", "-".repeat(64));
}
