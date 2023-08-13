use bevy::prelude::*;

pub fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
