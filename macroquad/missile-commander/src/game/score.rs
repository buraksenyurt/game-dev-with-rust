use crate::MAX_CITY_HEALTH;
use std::fmt::{Display, Formatter};

pub struct Score {
    pub total_hit: i32,
    pub total_point: i32,
    pub total_missed: i32,
    pub city_health: i32,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            total_point: 0,
            total_hit: 0,
            total_missed: 0,
            city_health: MAX_CITY_HEALTH,
        }
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "City Health {}, Player Hit/Missed({}/{}) Point {}",
            self.city_health, self.total_hit, self.total_missed, self.total_point
        )
    }
}
