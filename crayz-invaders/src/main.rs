use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Crayz Invaders".to_string(),
            width: 640.0,
            height: 480.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
