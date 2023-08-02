use std::fmt::Display;
use std::time::SystemTime;

pub struct Player {
    pub nick_name: String,
    pub enter_time: SystemTime,
    pub point: u32,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' - [{:?}]", self.nick_name, self.enter_time)
    }
}

pub struct Answer {
    pub title: String,
    pub is_correct: bool,
}

pub struct Question {
    pub title: String,
    pub answers: Vec<Answer>,
    pub point: u32,
}
