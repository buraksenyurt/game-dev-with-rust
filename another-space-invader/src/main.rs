use bevy::prelude::*;
use rand::random;

const COLUMN_COUNT: u16 = 8;
const ROW_COUNT: u16 = 4;
const SPACING: f32 = 40.0;
const INVADER_SPEED: f32 = 50.0;
const SHIFT_DOWN_AMOUNT: f32 = 32.0;
const PLAYER_SPEED: f32 = 200.0;
const BULLET_SPEED: f32 = 500.0;
const SHOOTING_COOLDOWN: f32 = 0.4;
// const INVADER_DIMENSION: (f32, f32) = (40.0, 32.0);

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
        app.add_plugins((InvaderPlugin, ResolutionPlugin, PlayerPlugin))
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
    commands.insert_resource(InvaderRouteResource {
        distance_from_boundary: 0f32,
        shift_down: false,
        direction: 1f32,
    });

    let red_texture = asset_server.load("red_enemy.png");
    let gold_texture = asset_server.load("gold_enemy.png");

    for col in 0..COLUMN_COUNT {
        for row in 0..ROW_COUNT {
            let texture = if random::<bool>() {
                red_texture.clone()
            } else {
                gold_texture.clone()
            };

            let position = Vec3::new(col as f32 * SPACING, row as f32 * SPACING, 0.0)
                - (Vec3::X * COLUMN_COUNT as f32 * SPACING * 0.5)
                - (Vec3::Y * ROW_COUNT as f32 * SPACING * 1.0)
                + (Vec3::Y * resolution.dimension.y * 0.5);

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(position)
                        .with_scale(Vec3::splat(resolution.pixel_ratio)),
                    texture: texture.clone(),
                    ..default()
                },
                Invader {},
            ));
        }
    }
}
struct InvaderPlugin;

impl Plugin for InvaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_invaders_system).add_systems(
            Update,
            (update_invaders_system, manage_invaders_route_system),
        );
    }
}

#[derive(Resource)]
struct InvaderRouteResource {
    direction: f32,
    shift_down: bool,
    distance_from_boundary: f32,
}
#[derive(Component)]
struct Invader;

fn update_invaders_system(
    mut query: Query<(&Invader, &mut Transform)>,
    mut manager: ResMut<InvaderRouteResource>,
    resolution: Res<Resolution>,
    time: Res<Time>,
) {
    for (_, mut transform) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * manager.direction * INVADER_SPEED;
        if transform.translation.x.abs() > resolution.dimension.x * 0.5 {
            manager.shift_down = true;
            manager.distance_from_boundary =
                resolution.dimension.x * manager.direction * 0.5 - transform.translation.x;
        }
    }
}

fn manage_invaders_route_system(
    mut query: Query<(&mut Invader, &mut Transform)>,
    mut manager: ResMut<InvaderRouteResource>,
) {
    if manager.shift_down {
        manager.direction *= -1f32;
        manager.shift_down = false;
        for (_, mut transform) in query.iter_mut() {
            transform.translation.x += manager.distance_from_boundary;
            transform.translation.y -= SHIFT_DOWN_AMOUNT;
        }
    }
}

#[derive(Component)]
struct Player {
    cooldown_timer: f32,
}

struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player_system)
            .add_systems(Update, update_player_system);
    }
}

fn setup_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<Resolution>,
) {
    let texture = asset_server.load("player.png");
    let position = Vec3::new(
        0.0,
        -(resolution.dimension.y * 0.5) + (resolution.pixel_ratio * 32.0),
        0.0,
    );
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_translation(position)
                .with_scale(Vec3::splat(resolution.pixel_ratio)),
            ..Default::default()
        },
        Player {
            cooldown_timer: 0.0,
        },
    ));
}

fn update_player_system(
    mut query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    resolution: Res<Resolution>,
) {
    let (mut player,mut transform) = query.single_mut();
    let mut horizontal = 0.0;
    if keys.pressed(KeyCode::ArrowLeft) {
        horizontal +=-1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        horizontal+=1.0;
    }
    transform.translation.x += horizontal * time.delta_seconds() * PLAYER_SPEED;

    let right_bound = resolution.dimension.x * 0.5;
    let left_bound = -resolution.dimension.x * 0.5;

    if transform.translation.x > right_bound {
        transform.translation.x = right_bound;
    }
    if transform.translation.x < left_bound {
        transform.translation.x = left_bound;
    }
}
