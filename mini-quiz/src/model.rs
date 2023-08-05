use chrono::NaiveTime;
use serde::Deserialize;
use std::fmt::Display;

pub struct Player {
    pub nick_name: String,
    pub enter_time: NaiveTime,
    pub point: u32,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' - [{:?}]", self.nick_name, self.enter_time)
    }
}

#[derive(Deserialize)]
pub struct Answer {
    pub title: String,
    pub is_correct: bool,
}

#[derive(Deserialize)]
pub struct Question {
    pub title: String,
    pub answers: Vec<Answer>,
    pub point: u32,
}
