use std::fmt::{Display, Formatter};

pub struct WinningCriteria {
    pub max_fighter: i16,
    pub max_warship: i16,
    pub max_bomber: i16,
}

impl Default for WinningCriteria {
    fn default() -> Self {
        WinningCriteria::new(16, 3, 12)
    }
}

impl Display for WinningCriteria {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Max F {} Max WS {} Max B {}",
            self.max_fighter, self.max_warship, self.max_bomber,
        )
    }
}

impl WinningCriteria {
    pub fn new(max_fighter: i16, max_warship: i16, max_bomber: i16) -> Self {
        Self {
            max_fighter,
            max_warship,
            max_bomber,
        }
    }
    pub async fn is_mission_accomplished(&self) -> bool {
        self.max_bomber <= 0 && self.max_warship <= 0 && self.max_fighter <= 0
    }
}
