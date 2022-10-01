use crate::constant::TILE_SIZE;
use bevy::prelude::*;

// Oyunu sisteminde yaptığımız gibi ASCII haritasını yöneten bu sistemi de
// plugin mantığında tekrardan tasarlayabiliriz.
pub struct TexturePlugin;

impl Plugin for TexturePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii_map);
    }
}

pub struct AsciiSheet(pub Handle<TextureAtlas>);

// Oyun sahasındaki nesneler için ascii haritası kullanılacak.
// Yani ASCII haritası üstüne yerleştirilen nesneleri oyun nesneleri olarak kullanacağız.
// Zemin, duvar, oyuncu, ejderha gibi
fn load_ascii_map(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("ascii_map.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::new(32., 32.), 16, 16);
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}

// ASCII tablosundan gelen parametre bilgilerine göre bir Entity oluşturulmasını sağlayan fonksiyon
// Örneğin oyuncu nesnesinin ASCII tablosundan çekilip oluşturulmasında kullanılıyor
pub fn spawn_sprite(
    commands: &mut Commands,
    ascii: &AsciiSheet,
    index: usize,
    color: Color,
    translation: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    sprite.color = color;

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}
