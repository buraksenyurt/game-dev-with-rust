use crate::data::model::*;

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
