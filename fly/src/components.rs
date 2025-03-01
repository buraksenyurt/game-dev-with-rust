use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub enum PlayerState {
    Idle,
    Running,
    Jumping,
}

#[derive(Component)]
pub struct PlayerAnimationHandles {
    pub idle: Handle<Image>,
    pub running: Handle<Image>,
    pub jumping: Handle<Image>,
}

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
