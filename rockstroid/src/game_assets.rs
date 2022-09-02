use crate::sprite::Sprite;
use crate::sprite_type::SpriteType;
use ggez::graphics::Image;
use ggez::{Context, GameResult};

pub struct GameAssets {
    player_image: Image,
    rock_image: Image,
    shot_image: Image,
}

impl GameAssets {
    pub fn new(ctx: &mut Context) -> GameResult<GameAssets> {
        let player_image = Image::new(ctx, "/ship.png")?;
        let rock_image = Image::new(ctx, "/asteroid.png")?;
        let shot_image = Image::new(ctx, "/shot.png")?;

        Ok(GameAssets {
            player_image,
            rock_image,
            shot_image,
        })
    }

    pub fn get_sprite_image(&self, sprite: &Sprite) -> &Image {
        match sprite.kind {
            SpriteType::Player => &self.player_image,
            SpriteType::Rock => &self.rock_image,
            SpriteType::Shot => &self.shot_image,
        }
    }
}
