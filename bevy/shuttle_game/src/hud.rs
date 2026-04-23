use crate::assets_manager::AssetsResource;
use bevy::app::App;
use bevy::prelude::*;

pub const SCORE_STYLE: Node = {
    let mut node = Node::DEFAULT;
    node.display = Display::Flex;
    node.flex_direction = FlexDirection::Row;
    node.justify_content = JustifyContent::SpaceBetween;
    node.align_items = AlignItems::Center;
    node.width = Val::Percent(100.);
    node.height = Val::Px(100.);
    node
};

#[derive(Component, Debug)]
pub struct ScoreboardPlugin;

#[derive(Component)]
pub struct ScoreText;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, span_scoreboard);
    }
}

fn span_scoreboard(mut commands: Commands, assets_resource: Res<AssetsResource>) {
    commands.spawn((
        Text::new("0"),
        TextFont {
            font: assets_resource.score_font.clone(),
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::srgb(0., 0., 0.)),
        TextLayout::new_with_justify(Justify::Center),
        SCORE_STYLE,
        ScoreText,
    ));
}
