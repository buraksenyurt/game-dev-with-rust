use inline_colorization::{color_red, color_reset};
use slam_dunk_manager::game::league::*;
use slam_dunk_manager::game::menus::*;
use slam_dunk_manager::prelude::contest::*;
use slam_dunk_manager::prelude::utility::*;
use slam_dunk_manager::prelude::view::*;
use std::io::{stdin, Read};
use std::str::FromStr;

fn main() {
    loop {
        print_main_menu();
        let input = get_input();
        let choose = MainMenu::from_str(&input);
        match choose {
            Ok(cmd) => match cmd {
                MainMenu::NewGame => {}
                MainMenu::LoadGame => {}
                MainMenu::TransferMarket => {}
                MainMenu::ExitGame => {
                    break;
                }
            },
            Err(reason) => {
                println!("{color_red}{reason}{color_reset}");
                continue;
            }
        }
    }

    // let mut league = create_league("NC2A Pre Session".to_string());
    // add_player_team("Academy Ist".to_string(), &mut league);
    // print_transfer_market(&league.transfer_market);
    // println!();
    // simulate_match_day(&mut league);
    // simulate_match_day(&mut league);
    // simulate_match_day(&mut league);
    // print_table(&mut league);
}
