use crate::resources::tile::Tile;
use std::ops::{Deref, DerefMut};

/*
   Ekran iki boyutlu bir array gibi düşünülürse zemin haritasını tutmamız gerekecektir.
*/
pub struct TileMap {
    pub mine_count: u16,
    pub height: u16,
    pub width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    // Boş bir zemin seti oluşturmak için kullanılan fonksiyon
    pub fn init(width: u16, height: u16) -> Self {
        // Aşağıdaki teknik oldukça etkili. High Order Function'lar kullanılarak
        // döngüye bile başvurmadan bir vector oluşturuluyor
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Empty).collect())
            .collect();
        Self {
            mine_count: 0,
            height,
            width,
            map,
        }
    }
}

// Map vektörünü dereferans etmek için Deref trait'ini uyguladık
impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
