use bevy::prelude::*;

pub struct AsciiSheet(pub Handle<TextureAtlas>);

// Oyun sahasındaki nesneler için ascii haritası kullanılacak.
// Yani ASCII haritası üstüne yerleştirilen nesneleri oyun nesneleri olarak kullanacağız.
// Zemin, duvar, oyuncu, ejderha gibi
pub fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("ascii_map.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::new(32., 32.), 16, 16);
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}
