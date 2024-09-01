mod assets_manager;
mod camera;
mod collision;
mod game_data;
mod hud;
mod log;
mod movement;
mod out_off_boundary;
mod pickup_crate;
mod planet;
mod planner;
mod shuttle;
mod state;

use crate::assets_manager::AssetsManagerPlugin;
use crate::camera::CameraPlugin;
use crate::collision::CollisionPlugin;
use crate::game_data::GameDataPlugin;
use crate::hud::ScoreboardPlugin;
use crate::movement::MovementPlugin;
use crate::out_off_boundary::OutOffBoundaryPlugin;
use crate::pickup_crate::PickupPlugin;
use crate::planet::PlanetPlugin;
use crate::planner::PlannerPlugin;
use crate::shuttle::ShuttlePlugin;
use crate::state::GameStatePlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.02353, 0.98039, 1.0)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.,
        })
        // User plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(GameDataPlugin)
        .add_plugins(ScoreboardPlugin)
        .add_plugins(PlanetPlugin)
        .add_plugins(AssetsManagerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(ShuttlePlugin)
        .add_plugins(PickupPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(OutOffBoundaryPlugin)
        .add_plugins(PlannerPlugin)
        .add_plugins(GameStatePlugin)
        //.add_plugins(LogPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
