mod common;
pub mod enemy;
mod events;
mod player;
mod score;
mod star;
mod systems;

use crate::enemy::resources::*;
use crate::score::resources::*;
use crate::star::resources::*;
use bevy::prelude::*;
use events::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_systems(
            Startup,
            (spawn_camera, spawn_player, spawn_enemies, spawn_stars),
        )
        .add_systems(
            Update,
            (
                control_player_movement,
                check_player_movement,
                enemy_movement,
                enemy_out_of_bound_check,
                check_enemy_movement,
                check_enemy_hit_player,
                refresh_everything,
                check_player_hits_star,
                update_score,
                tick_star_spawn_timer,
                enemy_spawn_timer,
                spawn_enemy_after_time_finished,
                exit_game,
                handle_game_over,
                update_high_score,
                handle_high_scores_updated,
                spawn_star_after_time_finished,
            ),
        )
        .run();
}
