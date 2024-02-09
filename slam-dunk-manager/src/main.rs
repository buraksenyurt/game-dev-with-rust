use inline_colorization::{color_cyan, color_green, color_magenta, color_red, color_reset};
use slam_dunk_manager::game::game::*;
use slam_dunk_manager::game::league::*;
use slam_dunk_manager::game::market::*;
use slam_dunk_manager::game::menus::*;
use slam_dunk_manager::game::team::check_team_name;
use slam_dunk_manager::prelude::utility::*;
use slam_dunk_manager::prelude::view::*;
use std::str::FromStr;

fn main() {
    let mut game = Game {
        current_state: GameState::Initial,
    };
    loop {
        print_main_menu();
        let input = get_input();
        let choose = MainMenu::from_str(&input);
        match choose {
            Ok(cmd) => match cmd {
                MainMenu::NewGame => {
                    game.current_state = GameState::MainMenu;
                    let mut league = create_league("NC2A Pre Session".to_string());
                    println!("{color_cyan}League has been created.");
                    loop {
                        game.current_state = GameState::TeamChoose;
                        println!("Please enter your team name...");
                        let team_name = get_input();
                        if !check_team_name(&team_name) {
                            continue;
                        }
                        let mut player_team =
                            add_player_team("Academy Ist".to_string(), &mut league);
                        println!("{team_name} has been added to league.");
                        println!("Please choose your team members.{color_reset}");
                        game.current_state = GameState::TransferMarket;
                        print_transfer_market(&league.transfer_market);
                        let mut player_count = 0;
                        while player_count <= 4 {
                            println!("{color_magenta}Please enter player's number. Be careful!{color_reset}");
                            if let Ok(n) = get_input().parse::<u16>() {
                                if let Some(p) = get_player(n, &league.transfer_market.players) {
                                    println!(
                                        "{color_green}{} has been added your team.{color_reset}",
                                        &p.full_name
                                    );
                                    player_team.players.push(p);
                                    player_count += 1;
                                } else {
                                    println!(
                                        "{color_red}Player not found in transfer market!{color_reset}"
                                    );
                                    continue;
                                }
                            } else {
                                println!("{color_red}Please enter a valid number!{color_reset}");
                                continue;
                            }
                        }
                        print_coach_team(&player_team);
                        break;
                    }
                }
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

    // simulate_match_day(&mut league);
    // simulate_match_day(&mut league);
    // simulate_match_day(&mut league);
    // print_table(&mut league);
}
