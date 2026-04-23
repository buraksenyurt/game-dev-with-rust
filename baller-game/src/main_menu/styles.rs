use bevy::color::palettes::css;
use bevy::prelude::*;

pub const DEFAULT_BUTTON_COLOR: Color = Color::srgb(0.15, 0.35, 0.65);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.65, 0.35, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.35, 0.35, 0.35);
pub const BUTTON_STYLE: Node = {
    let mut node = Node::DEFAULT;
    node.width = Val::Px(400.);
    node.height = Val::Px(80.);
    node.align_items = AlignItems::Center;
    node.justify_content = JustifyContent::Center;
    node
};
pub const MAIN_MENU_STYLE: Node = {
    let mut node = Node::DEFAULT;
    node.width = Val::Percent(100.);
    node.height = Val::Percent(100.);
    node.flex_direction = FlexDirection::Column;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.row_gap = Val::Px(10.);
    node.column_gap = Val::Px(10.);
    node
};
pub const TITLE_STYLE: Node = {
    let mut node = Node::DEFAULT;
    node.flex_direction = FlexDirection::Row;
    node.justify_content = JustifyContent::Center;
    node.width = Val::Px(600.);
    node.height = Val::Px(120.);
    node
};
pub const IMAGE_STYLE: Node = {
    let mut node = Node::DEFAULT;
    node.width = Val::Px(64.);
    node.height = Val::Px(64.);
    node.margin = UiRect::new(Val::Px(8.), Val::Px(8.), Val::Px(8.), Val::Px(8.));
    node
};

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> (TextFont, TextColor) {
    (
        TextFont {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 32.,
            ..default()
        },
        TextColor(Color::srgb(1., 1., 1.)),
    )
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> (TextFont, TextColor) {
    (
        TextFont {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 64.,
            ..default()
        },
        TextColor(Color::from(css::MIDNIGHT_BLUE)),
    )
}
