#[cfg(test)]
mod tests {
    use slam_dunk_manager::prelude::league::create_league;

    #[test]
    fn test_create_league() {
        let name = "NC2A Pre Session".to_string();
        let league = create_league(name.clone());
        assert_eq!(league.is_active, false);
        assert_eq!(league.name, name);
        assert_eq!(league.teams.len(), 7);
    }
}
