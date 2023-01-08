use crate::AsciiSheet;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}

fn spawn_player(mut commands: Commands, ascii_res: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(15);
    sprite.color = Color::GOLD;
    sprite.custom_size = Some(Vec2::splat(1.0));
    commands.spawn((
        SpriteSheetBundle {
            sprite,
            texture_atlas: ascii_res.0.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 100.),
                scale: Vec3::splat(0.1),
                ..default()
            },
            ..default()
        },
        Player,
    ));
}
