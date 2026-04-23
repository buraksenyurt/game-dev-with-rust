use chrono::Utc;
use clearscreen::clear;
use slam_dunk_manager::game_mngr::game::*;
use slam_dunk_manager::game_mngr::league::*;
use slam_dunk_manager::game_mngr::menus::*;
use slam_dunk_manager::prelude::colors::*;
use slam_dunk_manager::prelude::contest::create_schedule;
use slam_dunk_manager::prelude::utility::*;
use slam_dunk_manager::prelude::view::*;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut program_state = ProgramState::MainMenu;

    let game = Game::load("game_state.bin").await.unwrap_or_else(|_| Game {
        program_state: ProgramState::None,
        fixture: vec![],
    });

    'game_loop: loop {
        match program_state {
            ProgramState::MainMenu => {
                clear()?;
                print_main_menu();
                let input = get_input().ok_or("Input cannot be empty")?;
                let choose = MainMenu::from_str(&input);
                match choose {
                    Ok(cmd) => match cmd {
                        MainMenu::NewGame => {
                            program_state = ProgramState::InitNewGame;
                            continue 'game_loop;
                        }
                        MainMenu::LoadGame => {
                            continue 'game_loop;
                        }
                        MainMenu::TransferMarket => {
                            program_state = ProgramState::ShowTransferMarket;
                            continue 'game_loop;
                        }
                        MainMenu::ExitGame => {
                            program_state = ProgramState::Exit;
                            continue 'game_loop;
                        }
                    },
                    Err(reason) => {
                        println!("{color_red}{reason}{color_reset}");
                        continue;
                    }
                }
            }
            ProgramState::InitNewGame => {
                clear()?;
                let mut league = create_league();
                println!("{color_cyan}League has been created.");
                loop {
                    println!("{light_yellow}Please enter your team name...{color_reset}");
                    let team_name = get_input().ok_or("Input cannot be empty")?;
                    if !check_team_name(&team_name) {
                        continue;
                    }
                    let mut player_team = add_player_team(&team_name, &mut league);
                    println!("{team_name} has been added to league.");
                    println!("Please choose your team members.{color_reset}");
                    print_transfer_market(&league.transfer_market);
                    add_players_to_team(&mut league, &mut player_team);
                    print_coach_team(&player_team);
                    pause("Press any key to start creating schedule.");
                    let fixture = create_schedule(&league.teams, Utc::now());
                    pause("Schedule created. Press any key to show.");
                    print_fixture_by_paging(&fixture);
                    pause("End of schedule. Press any key to go menu.");
                    println!("The game is ready to launch");
                    game.save("game_state.bin").await?;
                    program_state = ProgramState::MainMenu;
                    break;
                }
            }
            ProgramState::Exit => {
                game.save("game_state.bin").await?;
                println!("Closing...");
                break 'game_loop;
            }
            ProgramState::ShowTransferMarket => {
                pause("Press any key to show current transfer market.");
                continue 'game_loop;
                //print_transfer_market(&league.transfer_market);
            }
            ProgramState::None => {}
        }
    }

    Ok(())
}
