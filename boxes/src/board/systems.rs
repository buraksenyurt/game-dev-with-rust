use crate::board::components::{Part, Position, Tile};
use crate::board::resources::*;
use crate::components::{get_world_position, Vector};
use crate::globals::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn load_board(mut commands: Commands, mut board: ResMut<ActiveBoard>) {
    let map: [[i32; 16]; 16] = [
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 2, 2, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        [1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        [1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 2, 2, 1, 1, 1, 1],
        [1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    ];

    board.tiles = HashMap::new();
    for row in 0..16 {
        for column in 0..16 {
            let tile_kind = match map[row][column] {
                0 => "Grass",
                1 => "Wall",
                2 => "Box",
                _ => "",
            };

            let value = Vector::new(row as i32, column as i32);
            let tile = commands
                .spawn((
                    Position { value },
                    Part {
                        kind: tile_kind.to_string(),
                    },
                    Tile,
                ))
                .id();
            board.tiles.insert(value, tile);
        }
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    mut assets: ResMut<AssetList>,
) {
    let texture = asset_server.load("sokoban_tilesheet.png");
    assets.0.push(texture.clone_untyped());
    let atlas = TextureAtlas::from_grid(texture, Vec2::splat(64.), 12, 8, None, None);
    let handle = texture_atlas.add(atlas);
    commands.insert_resource(GraphicsAssets { texture: handle });
}

pub fn spawn_parts(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Part), Added<Part>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position, part) in query.iter() {
        let sprite_idx = match part.kind.as_str() {
            "Wall" => 80,
            "Grass" => 82,
            "Box" => 1,
            _ => 0,
        };
        //info!("Sprite index {}", sprite_idx);
        let mut sprite = TextureAtlasSprite::new(sprite_idx);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        let v = get_world_position(&position, PART_Z);
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.texture.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}
