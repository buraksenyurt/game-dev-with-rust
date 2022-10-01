use crate::system::texture::AsciiSheet;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}

pub fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut player_texture = TextureAtlasSprite::new(14);
    player_texture.color = Color::rgb(255., 215., 0.);
    player_texture.custom_size = Some(Vec2::splat(0.1));
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: player_texture,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("AgentSmith"))
        .insert(Player);
}
