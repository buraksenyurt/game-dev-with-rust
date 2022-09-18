use macroquad::prelude::*;

// Oyuncunun yönettiği kutunun boyutları
pub const PLAYER_BOX_SIZE: Vec2 = Vec2::from_array([100., 25.]);
// Oyuncunun hareketinde kullanılan hız çarpanı
pub const PLAYER_SPEED: f32 = 600.;
// Oyun sahasının üst kısmında duracak olan blokların boyutları
pub const BLOCK_SIZE: Vec2 = Vec2::from_array([100., 40.]);
// Bloklar arası boşluklar için
pub const BLOCK_PADDING: f32 = 5.;
// Topun büyüklüğü ve hızı
pub const BALL_SIZE: f32 = 32.;
pub const BALL_SPEED: f32 = 400.;
