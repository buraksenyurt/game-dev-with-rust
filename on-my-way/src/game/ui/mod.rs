pub mod hud;

use crate::game::ui::hud::HeadUpDisplayPlugin;
use bevy::prelude::*;

pub struct GameUserInterfacePlugin;

impl Plugin for GameUserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HeadUpDisplayPlugin);
    }
}
