#[cfg(test)]
mod tests {
    use slam_dunk_manager::game::league::create_league;
    use slam_dunk_manager::prelude::market::get_player;

    #[test]
    pub fn test_get_player() {
        let name = "NC2A Pre Session".to_string();
        let league = create_league(name.clone());
        let actual = get_player(11, &league.transfer_market);
        assert!(actual.is_some());
    }
}
