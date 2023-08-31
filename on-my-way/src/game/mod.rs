use crate::game::live_data::LiveDataPlugin;
use crate::game::meteor::MeteorPlugin;
use crate::game::missile::MissilePlugin;
use crate::game::spaceship::SpaceshipPlugin;
use crate::game::station::FuelStationPlugin;
use crate::game::ui::GameUserInterfacePlugin;
use crate::systems::spawn_stars;
use crate::AppState;
use bevy::prelude::*;

pub mod live_data;
pub mod meteor;
pub mod missile;
pub mod spaceship;
pub mod station;
mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_plugins((
                SpaceshipPlugin,
                LiveDataPlugin,
                MeteorPlugin,
                FuelStationPlugin,
                MissilePlugin,
                GameUserInterfacePlugin,
            ));
    }
}
