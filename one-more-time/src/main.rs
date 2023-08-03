mod builder;
mod components;
mod constants;
mod enums;
mod resources;
mod systems;

use crate::constants::*;
use crate::resources::*;
use crate::systems::*;
use bevy::prelude::*;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "One More Time Blocky".into(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(GameState {
            balance: BALANCE_INIT_VALUE,
            cook_donut_count: 0,
            customers_inside: Vec::new(),
        })
        .add_systems(Startup, sys_setup)
        .add_systems(
            Update,
            (
                sys_player_movement,
                sys_spawn_donut,
                sys_claim_donut,
                sys_show_scoreboard,
                sys_customer_movement,
                sys_donut_movement,
                sys_leave_donut,
                sys_return_customers,
                sys_check_waiting_customers,
                sys_claim_waiting_customers,
            ),
        )
        .run();
}
