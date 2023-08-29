use crate::game::*;

pub fn toggle_playground(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    playground_state: Res<State<PlayGroundState>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        info!("Just Pressed the Pause button");
        if **playground_state == PlayGroundState::Running {
            commands.insert_resource(NextState(Some(PlayGroundState::Paused)));
            info!("Game in Paused state");
        }
        if **playground_state == PlayGroundState::Paused {
            commands.insert_resource(NextState(Some(PlayGroundState::Running)));
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
