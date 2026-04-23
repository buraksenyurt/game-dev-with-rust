use bevy::prelude::*;
pub const DEFAULT_BUTTON_COLOR: Color = Color::srgb_u8(0xb9, 0x3c, 0x28);
pub const DEFAULT_TEXT_COLOR: Color = Color::srgb_u8(0xf4, 0xcc, 0xcc);

pub fn game_over_menu_node() -> Node {
    Node {
        position_type: PositionType::Absolute,
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        ..default()
    }
}

pub fn game_over_menu_container_node() -> Node {
    Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(600.),
        height: Val::Px(600.),
        row_gap: Val::Px(24.),
        column_gap: Val::Px(24.),
        ..default()
    }
}

pub fn button_node() -> Node {
    Node {
        width: Val::Px(400.),
        height: Val::Px(60.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
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

pub fn get_final_score_text_font(asset_server: &Res<AssetServer>) -> TextFont {
    TextFont {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 42.,
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
