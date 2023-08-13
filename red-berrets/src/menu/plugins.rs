use crate::global::GameState;
use crate::menu::*;
use crate::utility::despawn_screen;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_setup_system)
            .add_systems(OnEnter(MenuState::Main), main_menu_setup_system)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            .add_systems(
                Update,
                (menu_action_system, button_interaction_system).run_if(in_state(GameState::Menu)),
            );
    }
}
