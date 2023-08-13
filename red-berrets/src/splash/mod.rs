use crate::utility::despawn_screen;
use crate::GameState;
use bevy::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), setup_splash_screen_system)
            .add_systems(Update, countdown_system.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashCountDownTimer(Timer);

fn setup_splash_screen_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let logo = asset_server.load("splash_logo.png");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(logo),
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Red Berrets - Mars Mission",
                TextStyle {
                    font_size: 40.,
                    ..default()
                },
            ));
        });

    commands.insert_resource(SplashCountDownTimer(Timer::from_seconds(
        5.,
        TimerMode::Once,
    )));
}

fn countdown_system(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashCountDownTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
