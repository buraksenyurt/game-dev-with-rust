#[cfg(test)]
mod tests {
    use chrono::Utc;
    use slam_dunk_manager::game_mngr::contest::create_schedule;
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
}
