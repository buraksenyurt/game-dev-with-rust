use crate::resources::tile_map::TileMap;
use bevy::app::App;
use bevy::log;
use bevy::prelude::*;

pub mod components;
mod resources;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
        info!("Board Plugin yüklendi");
    }
}

impl BoardPlugin {
    pub fn create_board() {
        // init fonksiyonu const olarak tanımlanmıştı.
        let mut tile_map = TileMap::init(20, 20);
        tile_map.setup_mines(32);
        #[cfg(feature = "debug")]
        info!("{}", tile_map.info())
    }
}
