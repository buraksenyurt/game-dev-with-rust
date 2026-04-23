use crate::game::*;

pub fn toggle_playground(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    playground_state: Res<State<PlayGroundState>>,
    mut next_state: ResMut<NextState<PlayGroundState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        info!("Just Pressed the Pause button");
        if *playground_state.get() == PlayGroundState::Running {
            next_state.set(PlayGroundState::Paused);
            info!("Game in Paused state");
        } else if *playground_state.get() == PlayGroundState::Paused {
            next_state.set(PlayGroundState::Running);
            info!("Game in Running state");
        }
    }
}

pub fn pause_game(mut playground_next_state: ResMut<NextState<PlayGroundState>>) {
    playground_next_state.set(PlayGroundState::Paused);
}

pub fn resume_game(mut playground_next_state: ResMut<NextState<PlayGroundState>>) {
    playground_next_state.set(PlayGroundState::Running);
}
