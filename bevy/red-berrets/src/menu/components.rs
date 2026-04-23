use bevy::prelude::Component;

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct OnSettingsMenuScreen;

#[derive(Component)]
pub struct OnSettingsControlsMenuScreen;

#[derive(Component)]
pub struct OnSettingsDifficultyMenuScreen;

#[derive(Component)]
pub struct OnCreditsMenuScreen;

#[derive(Component)]
pub struct SelectedOption;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Settings,
    SettingsControls,
    SettingsDifficulty,
    Credits,
    BackToMainMenu,
    BackToSettings,
    Quit,
}
