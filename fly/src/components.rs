use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Box;

// #[derive(Component)]
// pub struct Velocity(pub Vec3);

#[derive(Resource)]
pub struct BoxSpawningTimer(pub Timer);

#[derive(Component)]
pub struct StandardAnimation {
    pub timer: Timer,
}

impl Default for StandardAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}
