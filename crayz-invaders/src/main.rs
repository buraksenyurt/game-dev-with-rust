mod constant;

use crate::constant::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Crayz Invaders".to_string(),
            width: 480.0,
            height: 640.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(init_system)
        .run();
}

fn init_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let main_window = windows.get_primary_mut().unwrap();
    main_window.set_position(IVec2::new(0, 0));

    let (_width, height) = (main_window.width(), main_window.height());
    let bottom = -height / 2.;

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
