use crate::events::GameOverEvent;
use crate::game::live_data::LiveDataPlugin;
use crate::game::meteor::MeteorPlugin;
use crate::game::missile::MissilePlugin;
use crate::game::spaceship::SpaceshipPlugin;
use crate::game::station::FuelStationPlugin;
use crate::game::ui::GameUserInterfacePlugin;
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
        app.add_event::<GameOverEvent>().add_plugins((
            SpaceshipPlugin,
            LiveDataPlugin,
            MeteorPlugin,
            FuelStationPlugin,
            MissilePlugin,
            GameUserInterfacePlugin,
        ));
    }
}
