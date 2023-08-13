use crate::global::GameState;
use bevy::app::App;
use bevy::prelude::Plugin;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();
    }
}
