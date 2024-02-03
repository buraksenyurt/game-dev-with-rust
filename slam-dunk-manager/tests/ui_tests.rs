#[cfg(test)]
mod tests {
    use slam_dunk_manager::prelude::constants::*;
    use slam_dunk_manager::prelude::model::*;

    #[test]
    fn test_display_player_info() {
        let player = Player {
            id: 1,
            full_name: "Jordan Thomas".to_string(),
            position: Position::PowerForward,
            height: 218.49,
            energy: DEFAULT_PLAYER_ENERGY_LEVEL,
            stats: AverageStat {
                points_avg: 25.85,
                rebounds_avg: 9.05,
                assist_avg: 1.15,
                blocks_avg: 1.92,
                steals_avg: 0.4,
                turnovers_avg: 0.3,
            },
            transfer_fee: 7.57,
        };
        let actual = player.to_string();
        let expected =
            "# 01-Jordan Thomas            Power Forward 218.49cm 7.57 $".to_string();
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_display_player_stats() {
        let player = Player {
            id: 1,
            full_name: "Jordan Thomas".to_string(),
            position: Position::PowerForward,
            height: 218.49,
            energy: DEFAULT_PLAYER_ENERGY_LEVEL,
            stats: AverageStat {
                points_avg: 25.85,
                rebounds_avg: 9.05,
                assist_avg: 1.15,
                blocks_avg: 1.92,
                steals_avg: 0.4,
                turnovers_avg: 0.3,
            },
            transfer_fee: 7.57,
        };
        let actual = player.stats.to_string();
        let expected = "\tPnt      Reb      Ass      Blc      Ste      Trn     \n\t25.85    9.05     1.15     1.92     0.40     0.30    ".to_string();
        assert_eq!(actual, expected);
    }
}
