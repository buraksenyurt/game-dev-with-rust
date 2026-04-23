#[cfg(test)]
mod tests {
    use slam_dunk_manager::game_mngr::league::create_league;
    use slam_dunk_manager::prelude::market::take_player;

    #[test]
    pub fn test_get_player() {
        let league = create_league();
        let actual = take_player(11, &league.transfer_market.players);
        assert!(actual.is_some());
    }
}
