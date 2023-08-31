use bevy::prelude::*;
pub const DEFAULT_BUTTON_COLOR: &str = "#b93c28";
pub const DEFAULT_TEXT_COLOR: &str = "#f4cccc";
pub const GAME_OVER_MENU_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.position_type = PositionType::Absolute;
    style.display = Display::Flex;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.);
    style.height = Val::Percent(100.);
    style
};
pub const GAME_OVER_MENU_CONTAINER_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(600.);
    style.height = Val::Px(600.);
    style.row_gap = Val::Px(24.);
    style.column_gap = Val::Px(24.);
    style
};

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(400.);
    style.height = Val::Px(60.);
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style
};

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 64.,
        color: Color::hex(DEFAULT_TEXT_COLOR).unwrap(),
    }
}

pub fn get_final_score_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 42.,
        color: Color::hex(DEFAULT_TEXT_COLOR).unwrap(),
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 32.,
        color: Color::hex(DEFAULT_TEXT_COLOR).unwrap(),
    }
}
