use bevy::prelude::*;
pub const BACKGROUND_COLOR: Color = Color::srgba(0.25, 0.25, 0.25, 0.5);
pub fn hud_node() -> Node {
    Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        width: Val::Percent(100.),
        height: Val::Px(100.),
        ..default()
    }
}
// Left Side
pub fn lhs_node() -> Node {
    Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(200.),
        height: Val::Percent(60.),
        margin: UiRect::new(Val::Px(32.), Val::Px(0.), Val::Px(0.), Val::Px(0.)),
        ..default()
    }
}

// Center Side
pub fn chs_node() -> Node {
    Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(300.),
        height: Val::Percent(60.),
        margin: UiRect::new(Val::Px(32.), Val::Px(32.), Val::Px(0.), Val::Px(0.)),
        ..default()
    }
}

// Right Side
pub fn rhs_node() -> Node {
    Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(200.),
        height: Val::Percent(60.),
        margin: UiRect::new(Val::Px(0.), Val::Px(32.), Val::Px(0.), Val::Px(0.)),
        ..default()
    }
}
pub fn image_node_size() -> Node {
    Node {
        width: Val::Px(36.),
        height: Val::Px(36.),
        margin: UiRect::new(Val::Px(8.), Val::Px(8.), Val::Px(8.), Val::Px(8.)),
        ..default()
    }
}

pub fn get_text_font(asset_server: &Res<AssetServer>) -> TextFont {
    TextFont {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 36.0,
        ..default()
    }
}
pub fn get_text_color() -> TextColor {
    TextColor(Color::srgb(1., 1., 1.))
}
