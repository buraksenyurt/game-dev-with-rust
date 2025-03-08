use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub power: f32,
}
#[derive(Component)]
pub enum EnemyState {
    Idle,
    Walking,
}
#[derive(Component)]
pub struct EnemyAnimationHandles {
    pub idle: Handle<Image>,
    pub walking: Handle<Image>,
}

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
