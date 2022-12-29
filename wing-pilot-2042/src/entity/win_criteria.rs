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

impl WinningCriteria {
    pub fn new(max_fighter: i16, max_warship: i16, max_bomber: i16) -> Self {
        Self {
            max_fighter: max_fighter,
            max_warship,
            max_bomber: max_bomber,
        }
    }
}
