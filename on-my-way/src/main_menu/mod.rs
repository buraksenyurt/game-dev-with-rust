use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, main_menu);
    }
}

pub fn main_menu() {
    info!("Ana menü girişi")
}
