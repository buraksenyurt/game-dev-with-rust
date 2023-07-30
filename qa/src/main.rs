fn main() {
    let wilson = Player::new(23,"Can Kilod Van Dam");
    println!("{}", wilson.nick_name);
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
}

impl Player {
    pub fn new(id: i32, nick_name: &str) -> Self {
        Self { id, nick_name }
    }
}
