use bevy::prelude::*;
pub const BACKGROUND_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.5);
pub const HUD_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceBetween;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.);
    style.height = Val::Px(100.);
    style
};
// Left Side
pub const LHS_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(200.);
    style.height = Val::Percent(60.);
    style.margin = UiRect::new(Val::Px(32.), Val::Px(0.), Val::Px(0.), Val::Px(0.));
    style
};

// Center Side
pub const CHS_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(300.);
    style.height = Val::Percent(60.);
    style.margin = UiRect::new(Val::Px(32.), Val::Px(32.), Val::Px(0.), Val::Px(0.));
    style
};

// Right Side
pub const RHS_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(200.);
    style.height = Val::Percent(60.);
    style.margin = UiRect::new(Val::Px(0.), Val::Px(32.), Val::Px(0.), Val::Px(0.));
    style
};
pub const IMAGE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(36.);
    style.height = Val::Px(36.);
    style.margin = UiRect::new(Val::Px(8.), Val::Px(8.), Val::Px(8.), Val::Px(8.));
    style
};

pub fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 36.0,
        color: Color::rgb(1., 1., 1.),
    }
}
