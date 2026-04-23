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
    let logo: Handle<Image> = asset_server.load("splash_logo.png");

    commands
        .spawn((
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode::new(logo),
                Node {
                    width: Val::Px(256.0),
                    height: Val::Px(256.0),
                    margin: UiRect::bottom(Val::Px(16.0)),
                    ..default()
                },
            ));

            parent.spawn((
                Text::new("Red Berrets - Mars Mission"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });

    commands.insert_resource(SplashCountDownTimer(Timer::from_seconds(
        5.0,
        TimerMode::Once,
    )));
}

fn countdown_system(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashCountDownTimer>,
) {
    if timer.tick(time.delta()).just_finished() {
        game_state.set(GameState::Menu);
    }
}
