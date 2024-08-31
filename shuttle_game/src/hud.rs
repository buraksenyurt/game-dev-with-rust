use crate::assets_manager::AssetsResource;
use bevy::app::App;
use bevy::prelude::*;

pub const SCORE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceBetween;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.);
    style.height = Val::Px(100.);
    style
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
        TextBundle {
            style: SCORE_STYLE,
            text: Text {
                sections: vec![TextSection::new(
                    "0",
                    TextStyle {
                        font: assets_resource.score_font.clone(),
                        font_size: 36.0,
                        color: Color::srgb(0., 0., 0.),
                    },
                )],
                justify: JustifyText::Center,
                ..default()
            },
            ..default()
        },
        ScoreText,
    ));
}
