use bevy::prelude::*;
pub const DEFAULT_BUTTON_COLOR: &str = "#ff3322";
pub const DEFAULT_BACKGROUND_COLOR: &str = "#6a329f";
pub const DEFAULT_TEXT_COLOR: &str = "#f4cccc";
pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(400.);
    style.height = Val::Px(80.);
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style
};
pub const MAIN_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.);
    style.height = Val::Percent(100.);
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(10.);
    style.column_gap = Val::Px(10.);
    style
};
pub const TITLE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.width = Val::Px(600.);
    style.height = Val::Px(120.);
    style
};
pub const IMAGE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(64.);
    style.height = Val::Px(64.);
    style.margin = UiRect::new(Val::Px(8.), Val::Px(8.), Val::Px(8.), Val::Px(8.));
    style
};

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 32.,
        color: Color::hex(DEFAULT_TEXT_COLOR).unwrap(),
        ..default()
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 64.,
        color: Color::hex(DEFAULT_TEXT_COLOR).unwrap(),
        ..default()
    }
}
