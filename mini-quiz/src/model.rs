use chrono::NaiveTime;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

pub struct Player {
    pub nick_name: String,
    pub enter_time: NaiveTime,
    pub score: Score,
}

#[derive(Default)]
pub struct Score {
    pub total_point: u32,
    pub correct: u8,
    pub wrong: u8,
}
impl Display for Score {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Correct: {}, Wrong: {}, Total Point: {}",
            self.correct, self.wrong, self.total_point
        )
    }
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
