use crate::ascii::{spawn_sprite, AsciiSheet};
use crate::TILE_SIZE;
use bevy::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct TileMapPlugin;

#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
pub struct EncounterBuilder;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create);
    }
}

fn create(mut commands: Commands, ascii_res: Res<AsciiSheet>) {
    let mut tiles = Vec::new();
    let map_file = File::open("assets/schene_01.txt").expect("Map file not found");
    for (y, line) in BufReader::new(map_file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile = spawn_sprite(
                    &mut commands,
                    &ascii_res,
                    char as usize,
                    Color::GOLD,
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.),
                );
                // Eğer haritadaki # (duvar) yükleniyorsa o entity için
                // çarpışma bileşenini yükle.
                // Böylece player'ın movement(hareket) sisteminde,
                // TileCollider yüklü nesneleri sorgulayabilir ve duvarlardan geçilmesini
                // engelleyebiliriz.
                if char == '#' {
                    commands.entity(tile).insert(TileCollider);
                }
                if char == '~' {
                    commands.entity(tile).insert(EncounterBuilder);
                }
                tiles.push(tile);
            }
        }
    }

    commands
        .spawn(Name::new("Map"))
        .insert(ComputedVisibility::default())
        .insert(Visibility::VISIBLE)
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}
