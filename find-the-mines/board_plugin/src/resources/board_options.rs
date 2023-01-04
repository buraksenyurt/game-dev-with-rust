/*
    Oyun sahasına ait bazı parametrik argümanlar bu dosyada yer alıyor.
    Zemin ebatlar, oyun sahasının konumlanması vb.
    app nesnesin bu ayarlar bir resource olarak ilave edilecekler.
*/
use bevy::prelude::{Resource, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize {
    Fixed(f32),
    Adaptive { min: f32, max: f32 },
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive { min: 10., max: 50. }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoardPosition {
    Centered { offset: Vec3 },
    Custom(Vec3),
}

impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered {
            offset: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub struct BoardOptions {
    pub map_size: (u16, u16),
    pub mine_count: u16,
    pub position: BoardPosition,
    pub tile_size: TileSize,
    pub tile_padding: f32,
    pub safe_start: bool,
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            map_size: (15, 15),
            mine_count: 30,
            position: Default::default(),
            tile_size: Default::default(),
            tile_padding: 0.,
            safe_start: false,
        }
    }
}
