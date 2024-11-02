use bevy::prelude::*;

const COLUMN_COUNT: u16 = 6;
const ROW_COUNT: u16 = 4;
const SPACING: f32 = 40.0;

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

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((InvaderPlugin, ResolutionPlugin))
            .add_systems(Startup, setup_system);
    }
}

#[derive(Resource)]
struct Resolution {
    pub dimension: Vec2,
    pub pixel_ratio: f32,
}
fn setup_resolution_system(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    commands.insert_resource(Resolution {
        dimension: Vec2::new(window.width(), window.height()),
        pixel_ratio: 0.75,
    })
}
struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_resolution_system);
    }
}
fn setup_invaders_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<Resolution>,
) {
    let texture = asset_server.load("red_enemy.png");
    for col in 0..COLUMN_COUNT {
        for row in 0..ROW_COUNT {
            let position = Vec3::new(col as f32 * SPACING, row as f32 * SPACING, 0.0);
            commands.spawn(SpriteBundle {
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(resolution.pixel_ratio)),
                texture: texture.clone(),
                ..default()
            });
        }
    }
}
struct InvaderPlugin;

impl Plugin for InvaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_invaders_system);
    }
}
