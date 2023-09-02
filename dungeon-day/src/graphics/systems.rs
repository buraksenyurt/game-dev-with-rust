use crate::asset::resources::*;
use crate::board::components::*;
use crate::graphics::resources::*;
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
        let v = Vec3::new(
            TILE_SIZE * position.value.x as f32,
            TILE_SIZE * position.value.y as f32,
            0.,
        );
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.texture.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}
