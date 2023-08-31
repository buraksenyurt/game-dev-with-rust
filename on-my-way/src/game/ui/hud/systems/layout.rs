use bevy::prelude::*;

use crate::game::ui::hud::components::*;
use crate::game::ui::hud::styles::*;

pub fn spawn_head_up_display(mut commands: Commands, asset_server: Res<AssetServer>) {
    build(&mut commands, &asset_server);
}

fn build(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: HUD_STYLE,
                ..default()
            },
            HeadUpDisplay {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: LHS_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/spaceShips_001.png").into(),
                        ..default()
                    });
                    parent.spawn((
                        TextBundle {
                            style: Style { ..default() },
                            text: Text {
                                sections: vec![TextSection::new("0", get_text_style(asset_server))],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ScoreText {},
                    ));
                });

            parent
                .spawn(NodeBundle {
                    style: CHS_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/spaceMeteors_002.png").into(),
                        ..default()
                    });
                    parent.spawn((
                        TextBundle {
                            style: Style { ..default() },
                            text: Text {
                                sections: vec![TextSection::new("0", get_text_style(asset_server))],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        MeteorInfoText {},
                    ));
                });

            parent
                .spawn(NodeBundle {
                    style: RHS_STYLE,
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/spaceBuilding_002.png").into(),
                        ..default()
                    });

                    parent.spawn((
                        TextBundle {
                            style: Style { ..default() },
                            text: Text {
                                sections: vec![TextSection::new("0", get_text_style(asset_server))],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
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
        commands.entity(entity).despawn_recursive();
    }
}
