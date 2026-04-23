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

#[derive(Component)]
pub struct Barrel {
    pub power: f32,
}
#[derive(Component)]
pub enum BarrelState {
    Idle,
    Walking,
}
#[derive(Component)]
pub struct BarrelAnimationHandles {
    pub idle: Handle<Image>,
    pub walking: Handle<Image>,
}

#[derive(Component)]
pub struct BarrelDriver {
    pub timer: Timer,
    pub state: BarrelMoveState,
    pub direction: Vec2,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BarrelMoveState {
    Moving,
    Stopped,
}

impl Default for BarrelDriver {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
        Self {
            timer,
            state: BarrelMoveState::Stopped,
            direction: Vec2::ZERO,
        }
    }
}
