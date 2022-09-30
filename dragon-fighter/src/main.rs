mod constant;

use crate::constant::CLEAR_COLOR;
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(WindowDescriptor {
            width: 1600.,
            height: 900.,
            title: "Dragon Fighter".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
