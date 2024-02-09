use std::io::stdin;
pub fn get_input() -> Option<String> {
    let mut input = String::new();
    if stdin().read_line(&mut input).is_ok() {
        return Some(input.trim().to_string());
    }
    None
}
