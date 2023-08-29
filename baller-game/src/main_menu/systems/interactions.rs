use crate::main_menu::components::*;
use crate::main_menu::styles::{DEFAULT_BUTTON_COLOR, HOVERED_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;

pub fn pressed_play_game_button(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = query.get_single_mut() {
        match interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                info!("In game mod");
                app_state.set(AppState::Game);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = DEFAULT_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn pressed_quit_game_button(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = query.get_single_mut() {
        match interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = DEFAULT_BUTTON_COLOR.into();
            }
        }
    }
}
