use slam_dunk_manager::game::league::{add_player_team, create_league};
use slam_dunk_manager::prelude::r#match::simulate_match_day;
use slam_dunk_manager::prelude::terminal::{print_table, print_transfer_market};

fn main() {
    let mut league = create_league("NC2A Pre Session".to_string());
    add_player_team("Academy Ist".to_string(), &mut league);
    //print_transfer_market(&league.transfer_market);
    simulate_match_day(&mut league);
    simulate_match_day(&mut league);
    simulate_match_day(&mut league);
    print_table(&mut league);
}
