use bevy::prelude::*;
#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub(crate) direction: Vec2, // Kırmızı topların anlık konumlarını saklamak için eklendi
}

#[derive(Component)]
pub struct Star {}
