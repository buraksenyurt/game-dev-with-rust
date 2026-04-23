use bevy::prelude::States;

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Settings,
    SettingsControls,
    SettingsDifficulty,
    Credits,
    #[default]
    Disabled,
}
