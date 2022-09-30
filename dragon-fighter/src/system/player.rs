use crate::system::texture::AsciiSheet;
use bevy::prelude::*;

pub fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut plyr = TextureAtlasSprite::new(14);
    plyr.color = Color::rgb(255., 215., 0.);
    plyr.custom_size = Some(Vec2::splat(0.1));
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: plyr,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("AgentSmith"));
}
