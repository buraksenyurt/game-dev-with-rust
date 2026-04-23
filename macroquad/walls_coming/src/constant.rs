use macroquad::prelude::*;

// Oyuncunun yönettiği kutunun boyutları
pub const PLAYER_BOX_SIZE: Vec2 = Vec2::from_array([100., 20.]);
// Oyuncu bloğunun uzama ve kısaltmasın kullanılan varsayılan ölçü
pub const DEFAULT_ELONGATION: f32 = 25.;
// Oyuncunun hareketinde kullanılan hız çarpanı
pub const PLAYER_SPEED: f32 = 600.;
// Oyun sahasının üst kısmında duracak olan blokların boyutları
pub const BLOCK_SIZE: Vec2 = Vec2::from_array([80., 40.]);
// Bloklar arası boşluklar için
pub const BLOCK_PADDING: f32 = 5.;
// Topun büyüklüğü ve hızı
pub const BALL_SIZE: f32 = 32.;
pub const BALL_SPEED: f32 = 400.;
// Yavaşlama bonusuna denk gelirsek kullanılacak varsayılan azaltma hızı
pub const CAPTAIN_SLOW_SPEED: f32 = 150.;
// Topu yavaşlatan powerup'ın ne kadar birim devrede kalacağını ifade eder
pub const CAPTAIN_SLOW_LIFETIME:usize=500;
// Blokların dizildiği grid'e ait satır ve sütun sayıları
pub const ROW_COUNT: usize = 5;
pub const COLUMN_COUNT: usize = 8;
