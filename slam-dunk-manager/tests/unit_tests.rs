#[cfg(test)]
mod tests {
    use slam_dunk_manager::prelude::utility::get_input;

    #[test]
    pub fn test_terminal_input_is_number() {
        println!("Please enter player's number");
        let number = get_input().parse::<u16>();
        assert!(number.is_ok());
    }
}
