use super::*;
#[derive(Resource)]
pub struct FuelCheckTimer {
    pub timer: Timer,
}
impl Default for FuelCheckTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(FUEL_DECREASE_PERIOD, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct MissileLaunchCheckTimer {
    pub timer: Timer,
}
impl Default for MissileLaunchCheckTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(MISSILE_003_LAUNCHABLE_POINT, TimerMode::Repeating),
        }
    }
}
