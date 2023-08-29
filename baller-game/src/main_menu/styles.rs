use bevy::prelude::*;

pub const DEFAULT_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(400.);
    style.height = Val::Px(80.);
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style
};
