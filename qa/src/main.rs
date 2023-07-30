use std::fmt::{Display, Formatter};

fn main() {
    let wilson = Player::new(
        23,
        "Can Kilod Van Dam",
        Level::Pro(Score { win: 23, lose: 5 }),
    );
    let revenue = match wilson.level {
        Level::Beginner(s) => match s.win {
            20..=50 => 100,
            _ => 125,
        },
        Level::Pro(s) => match s.lose {
            0..=10 => 250,
            11..=20 => 100,
            _ => 0,
        },
        Level::Veteran(_) | Level::Elit => 250,
    };

    println!(
        "{}({}) isimli oyuncunun ödülü {} coin",
        wilson.nick_name, wilson.level, revenue
    );

    //println!("{}", wilson.nick_name);
}

pub struct Player<'a> {
    pub id: i32,
    pub nick_name: &'a str,
    pub level: Level,
}

impl<'a> Player<'a> {
    pub fn new(id: i32, nick_name: &'a str, level: Level) -> Self {
        Self {
            id,
            nick_name,
            level,
        }
    }
}

pub enum Level {
    Beginner(Score),
    Pro(Score),
    Veteran(Score),
    Elit,
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Beginner(s) => {
                write!(f, "({:?})", s)
            }
            Level::Pro(s) => {
                write!(f, "({:?})", s)
            }
            Level::Veteran(s) => {
                write!(f, "({:?})", s)
            }
            Level::Elit => {
                write!(f, "(Elit)")
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Score {
    pub win: u16,
    pub lose: u16,
}
