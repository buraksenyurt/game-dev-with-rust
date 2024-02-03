use crate::data::model::*;
use std::cmp::Ordering::Equal;

pub fn print_transfer_market(market: &TransferMarket) {
    println!("{}", "-".repeat(64));
    println!(
        "{} TRANSFER MARKET ({:.2}) Million ${}",
        "-".repeat(15),
        market.total_value,
        "-".repeat(15)
    );
    println!("{}", "-".repeat(64));
    for p in market.players.iter() {
        println!("{p}");
        println!("{}", p.stats);
    }
    println!("{}", "-".repeat(64));
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
    println!("{}", "-".repeat(64));
    println!("{}{:<24}{}", "-".repeat(20), league.name, "-".repeat(20));
    println!("{}", "-".repeat(64));
    println!(
        " {:<2}   {:<20} {:<3} {:<3} {:<3} {:<8} {:<8} {:<9}",
        " ", "Team Name", "GP", "W", "L", "Points+", "Points-", "Diff"
    );
    let mut i = 1;
    for team in league.teams.iter() {
        println!(
            " {:<2} - {:<20} {:<3} {:<3} {:<3} {:<8} {:<8} {:<9}",
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
    println!("{}", "-".repeat(64));
}
