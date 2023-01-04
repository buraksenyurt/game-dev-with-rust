use crate::resources::tile_map::TileMap;
use crate::resources::BoardOptions;
use bevy::app::App;
use bevy::prelude::*;

pub mod components;
pub mod resources;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
        info!("Board Plugin yüklendi");
    }
}

impl BoardPlugin {
    // Fonksiyon parametrik yapısı BoardOptions seçeneği eklendikten sonra değiştirildi
    pub fn create_board(board_options: Option<Res<BoardOptions>>) {
        // Artık oyun sahasını inşa ederken kullanılacak seçeneler BoardOptions veri yapısından
        // alınıp kullanılabiliyor.
        let options = match board_options {
            Some(o) => o.clone(),
            None => BoardOptions::default(),
        };
        // init fonksiyonu const olarak tanımlanmıştı.
        let mut tile_map = TileMap::init(options.map_size.0, options.map_size.1);
        tile_map.setup_mines(options.mine_count);
        #[cfg(feature = "debug")]
        info!("{}", tile_map.info())
    }
}
