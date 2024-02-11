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
    'menu_loop: loop {
        clear().unwrap();
        print_main_menu();
        let input = get_input().unwrap();
        let choose = MainMenu::from_str(&input);

        match choose {
            Ok(cmd) => match cmd {
                MainMenu::NewGame => {
                    game.current_state = GameState::NewGame;
                    break 'menu_loop;
                }
                MainMenu::LoadGame => {
                    game.current_state = GameState::Load;
                    break 'menu_loop;
                }
                MainMenu::TransferMarket => {
                    game.current_state = GameState::TransferMarket;
                    break 'menu_loop;
                }
                MainMenu::ExitGame => {
                    game.current_state = GameState::Exit;
                    break 'menu_loop;
                }
            },
            Err(reason) => {
                println!("{color_red}{reason}{color_reset}");
                continue 'menu_loop;
            }
        }
    }

    'main_loop: loop {
        match game.current_state {
            GameState::Initial => {}
            GameState::MainMenu => {}
            GameState::TransferMarket => {}
            GameState::NewGame => {
                clear().unwrap();
                let mut league = create_league();
                println!("{color_cyan}League has been created.");
                loop {
                    game.current_state = GameState::TeamChoose;
                    println!("{color_bright_yellow}Please enter your team name...{color_reset}");
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
                    let fixture = create_schedule(&league.teams, Utc::now());
                    pause("Schedule created. Press any key to show.");
                    print_fixture(&fixture);
                    pause("End of schedule. Press any key to go menu.");
                    game.current_state = GameState::ReadyToLaunch;
                    continue 'main_loop;
                }
            }
            GameState::TeamChoose => {}
            GameState::ReadyToLaunch => {
                println!("The game is ready to launch");
                break 'main_loop;
            }
            GameState::Load => {
                println!("Load last saved game!");
                break 'main_loop;
            }
            GameState::Exit => {
                println!("Closing...");
                break 'main_loop;
            }
        }
    }
}
