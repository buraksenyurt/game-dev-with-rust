use slam_dunk_manager::game::league::create_league;
use slam_dunk_manager::prelude::terminal::print_transfer_market;

fn main() {
    let league = create_league("NC2A Pre Session".to_string());
    print_transfer_market(&league.transfer_market);
}
