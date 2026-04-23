use bevy::prelude::*;

use crate::game::ui::hud::components::*;
use crate::game::ui::hud::styles::*;

pub fn spawn_head_up_display(mut commands: Commands, asset_server: Res<AssetServer>) {
    build(&mut commands, &asset_server);
}

fn build(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((
            hud_node(),
            HeadUpDisplay {},
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    lhs_node(),
                    BackgroundColor(BACKGROUND_COLOR),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        image_node_size(),
                        ImageNode::new(asset_server.load("sprites/spaceShips_001.png")),
                    ));
                    parent.spawn((
                        Text::new("0"),
                        get_text_font(asset_server),
                        get_text_color(),
                        ScoreText {},
                    ));
                });

            parent
                .spawn((
                    chs_node(),
                    BackgroundColor(BACKGROUND_COLOR),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        image_node_size(),
                        ImageNode::new(asset_server.load("sprites/spaceMeteors_002.png")),
                    ));
                    parent.spawn((
                        Text::new("0"),
                        get_text_font(asset_server),
                        get_text_color(),
                        MeteorInfoText {},
                    ));
                });

            parent
                .spawn((
                    rhs_node(),
                    BackgroundColor(BACKGROUND_COLOR),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        image_node_size(),
                        ImageNode::new(asset_server.load("sprites/spaceBuilding_002.png")),
                    ));

                    parent.spawn((
                        Text::new("0"),
                        get_text_font(asset_server),
                        get_text_color(),
                        FuelText {},
                    ));
                });
        })
        .id();

    hud_entity
}

pub fn despawn_head_up_display(
    mut commands: Commands,
    hud_query: Query<Entity, With<HeadUpDisplay>>,
) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn();
    }
}
