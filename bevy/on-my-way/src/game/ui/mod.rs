mod game_over_menu;
pub mod hud;

use crate::game::ui::game_over_menu::GameOverMenuPlugin;
use crate::game::ui::hud::HeadUpDisplayPlugin;
use bevy::prelude::*;

pub struct GameUserInterfacePlugin;

impl Plugin for GameUserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HeadUpDisplayPlugin, GameOverMenuPlugin));
    }
}
