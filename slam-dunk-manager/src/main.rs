use chrono::Utc;
use clearscreen::clear;
use inline_colorization::*;
use slam_dunk_manager::game_mngr::game::*;
use slam_dunk_manager::game_mngr::league::*;
use slam_dunk_manager::game_mngr::menus::*;
use slam_dunk_manager::prelude::contest::create_schedule;
use slam_dunk_manager::prelude::utility::*;
use slam_dunk_manager::prelude::view::*;
use std::str::FromStr;

fn main() {
    let mut game = Game {
        current_state: GameState::Initial,
    };
    loop {
        clear().unwrap();
        print_main_menu();
        let input = get_input().unwrap();
        let choose = MainMenu::from_str(&input);
        match choose {
            Ok(cmd) => match cmd {
                MainMenu::NewGame => {
                    game.current_state = GameState::MainMenu;
                    clear().unwrap();
                    let mut league = create_league();
                    println!("{color_cyan}League has been created.");
                    loop {
                        game.current_state = GameState::TeamChoose;
                        println!(
                            "{color_bright_yellow}Please enter your team name...{color_reset}"
                        );
                        let team_name = get_input().unwrap();
                        if !check_team_name(&team_name) {
                            continue;
                        }
                        let mut player_team = add_player_team(&team_name, &mut league);
                        println!("{team_name} has been added to league.");
                        println!("Please choose your team members.{color_reset}");
                        game.current_state = GameState::TransferMarket;
                        print_transfer_market(&league.transfer_market);
                        add_players_to_team(&mut league, &mut player_team);
                        print_coach_team(&player_team);
                        pause("Press any key to start creating schedule.");
                        let _fixture = create_schedule(&league.teams, Utc::now());
                        pause("Schedule created. Press any key to show");
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
}
