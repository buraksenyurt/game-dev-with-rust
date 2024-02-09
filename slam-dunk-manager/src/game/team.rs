pub fn check_team_name(input: &str) -> bool {
    input.len() > 5 || input.len() < 15
}
