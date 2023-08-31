use bevy::prelude::*;
#[derive(Component, Clone)]
pub struct Missile {
    pub direction: Vec2,
    pub speed: f32,
    pub width: f32,
    pub fuel_cost: f32,
    pub disposable: bool,
    pub location: Vec3,
}
#[derive(Component)]
pub struct ExplosionAnimation {
    pub first: usize,
    pub last: usize,
    pub disposable: bool,
}
#[derive(Component, Deref, DerefMut)]
pub struct ExplosionAnimationTimer(pub Timer);
