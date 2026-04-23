use bevy::prelude::*;
pub const DEFAULT_BUTTON_COLOR: Color = Color::srgb_u8(0xb9, 0x3c, 0x28);
pub const DEFAULT_TEXT_COLOR: Color = Color::srgb_u8(0xf4, 0xcc, 0xcc);

pub fn button_node() -> Node {
    Node {
        width: Val::Px(400.),
        height: Val::Px(80.),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

pub fn main_menu_node() -> Node {
    Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Px(10.),
        column_gap: Val::Px(10.),
        ..default()
    }
}

pub fn main_menu_container_node() -> Node {
    Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(1920.),
        height: Val::Px(1080.),
        row_gap: Val::Px(8.),
        column_gap: Val::Px(8.),
        ..default()
    }
}

pub fn get_button_text_font(asset_server: &Res<AssetServer>) -> TextFont {
    TextFont {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 32.,
        ..default()
    }
}

pub fn get_title_text_font(asset_server: &Res<AssetServer>) -> TextFont {
    TextFont {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 64.,
        ..default()
    }
}
