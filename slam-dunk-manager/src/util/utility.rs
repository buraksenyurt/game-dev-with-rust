use std::io::stdin;
pub fn get_input() -> String {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {}
    }
    input.trim().to_string()
}
