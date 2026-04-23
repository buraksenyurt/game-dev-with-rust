use bevy::math::Vec2;

pub const GROUND_LEVEL: f32 = -50.0;
pub const GROUND_TILE_COUNT: usize = 52;
pub const VERTICAL_TILE_COUNT: usize = 15;
pub const PLAYER_START_X: f32 = -400.0;
pub const PLAYER_SPEED: f32 = 180.0;
pub const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 32.0);
pub const BARREL_SIZE: Vec2 = Vec2::new(48.0, 48.0);
pub const BARREL_WALK_SPEED: f32 = 100.0;
pub const JUMP_FORCE: f32 = 500.0;
pub const GRAVITY_FORCE: f32 = 100.0;
pub const BOX_GRAVITY_FORCE: f32 = 10.0;
pub const TILE_SIZE: f32 = 24.0;
