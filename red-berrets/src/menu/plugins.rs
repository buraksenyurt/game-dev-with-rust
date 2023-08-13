use crate::global::{Difficulty, GameState};
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
            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup_system)
            .add_systems(
                OnExit(MenuState::Settings),
                despawn_screen::<OnSettingsMenuScreen>,
            )
            .add_systems(
                Update,
                (menu_action_system, button_interaction_system).run_if(in_state(GameState::Menu)),
            )
            .add_systems(
                OnEnter(MenuState::SettingsDifficulty),
                difficulty_settings_menu_setup_system,
            )
            .add_systems(
                Update,
                setting_button.run_if(in_state(MenuState::SettingsDifficulty)),
            )
            .add_systems(
                OnExit(MenuState::SettingsDifficulty),
                despawn_screen::<OnSettingsDifficultyMenuScreen>,
            );
    }
}

fn setting_button(
    interaction_query: Query<
        (&Interaction, &Difficulty, Entity),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, diff, entity) in &interaction_query {
        if *interaction == Interaction::Pressed {
            info!("{:?}", diff);
        }
    }
}
