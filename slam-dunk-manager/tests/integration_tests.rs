#[cfg(test)]
mod tests {
    use chrono::Utc;
    use slam_dunk_manager::game_mngr::contest::create_schedule;
    use slam_dunk_manager::prelude::game::{Game, GameState};
    use slam_dunk_manager::prelude::league::{add_player_team, create_league};

    #[test]
    fn test_create_league() {
        let league = create_league();
        assert_eq!(league.is_active, false);
        assert_eq!(league.teams.len(), 7);
    }

    #[test]
    fn test_add_players_team() {
        let mut league = create_league();
        assert_eq!(league.teams.len(), 7);
        add_player_team("Academy Tokyo", &mut league);
        assert_eq!(league.teams.len(), 8);
    }

    #[test]
    fn test_create_schedule() {
        let mut league = create_league();
        assert_eq!(league.teams.len(), 7);
        add_player_team("Academy Tokyo", &mut league);
        assert_eq!(league.teams.len(), 8);
        let fixture = create_schedule(&league.teams, Utc::now());
        assert_eq!(fixture.len(), 7);
    }

    #[tokio::test]
    async fn test_save_game_and_load_data() {
        let mut league = create_league();
        assert_eq!(league.teams.len(), 7);
        add_player_team("Academy Tokyo", &mut league);
        assert_eq!(league.teams.len(), 8);
        let fixture = create_schedule(&league.teams, Utc::now());
        assert_eq!(fixture.len(), 7);
        let game = Game {
            program_state: GameState::Initial,
            fixture,
        };
        assert!(game.save("GamesData.bin").await.is_ok());

        let game = Game::load("GamesData.bin").await;
        assert!(game.is_ok());
        assert_eq!(game.unwrap().fixture.len(), 7);
    }
}
