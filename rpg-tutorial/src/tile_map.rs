use crate::ascii::{spawn_sprite, AsciiSheet};
use crate::TILE_SIZE;
use bevy::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create);
    }
}

fn create(mut commands: Commands, ascii_res: Res<AsciiSheet>) {
    let map_file = File::open("assets/schene_01.txt").expect("Map file not found");
    for (y, line) in BufReader::new(map_file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                spawn_sprite(
                    &mut commands,
                    &ascii_res,
                    char as usize,
                    Color::GOLD,
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.),
                );
            }
        }
    }
}
