use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Guardians of Space Invaders"),
                        resolution: Vec2::new(480f32, 482f32).into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default()),
            GamePlugin,
        ))
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_system);
    }
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
