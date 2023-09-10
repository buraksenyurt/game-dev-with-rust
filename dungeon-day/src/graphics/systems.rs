use crate::asset::resources::*;
use crate::board::components::*;
use crate::graphics::resources::*;
use crate::parts::components::Part;
use crate::utility::get_world_position;
use bevy::prelude::*;

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

pub fn spawn_tiles(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position) in query.iter() {
        let mut sprite = TextureAtlasSprite::new(80);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        //sprite.color = Color::rgb_u8(61, 133, 198);
        let v = get_world_position(&position, TILE_Z);
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.texture.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}

pub fn spawn_part(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Part), Added<Part>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position, part) in query.iter() {
        let sprite_idx = match part.kind.as_str() {
            "Prince of Persia" => 72,
            "NPC" => 68,
            // "Wall" => 80,
            // "Grass" => 82,
            _ => 1,
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

pub fn update_part_position(
    mut query: Query<(&Position, &mut Transform), With<Part>>,
    time: Res<Time>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    let mut animating = false;
    for (position, mut transform) in query.iter_mut() {
        let target = get_world_position(&position, PART_Z);
        let distance = (target - transform.translation).length();
        if distance > POSITION_TOLERANCE {
            transform.translation = transform
                .translation
                .lerp(target, PART_SPEED * time.delta_seconds());
        } else {
            transform.translation = target;
        }
    }
    if animating {
        ev_wait.send(super::GraphicsWaitEvent);
    }
}
