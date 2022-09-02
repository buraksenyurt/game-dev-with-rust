use crate::constant::*;
use crate::sprite::Sprite;
use crate::sprite_type::SpriteType;
use ggez::mint::{Point2, Vector2};

// Oyundaki nesneleri oluşturmak için bazı yardımcı fonksiyonlar.
// Esasında Player, Rock ve Shot için ayrı birer veri yapısı da tasarlanabilir miydi?
// Şu haliyle her bir Sprite için birer constructor yazmış gibiyiz.

pub fn create_player() -> Sprite {
    Sprite::new(
        SpriteType::Player,
        Point2 { x: 0., y: 0. },
        Vector2 { x: 0., y: 0. },
        0.,
        PLAYER_BOX_SIZE,
        PLAYER_LIFE,
        0.,
    )
}

pub fn create_rock() -> Sprite {
    Sprite::new(
        SpriteType::Rock,
        Point2 { x: 0., y: 0. },
        Vector2 { x: 0., y: 0. },
        0.,
        ROCK_BOX_SIZE,
        ROCK_LIFE,
        0.,
    )
}

pub fn create_shot() -> Sprite {
    Sprite::new(
        SpriteType::Shot,
        Point2 { x: 0., y: 0. },
        Vector2 { x: 0., y: 0. },
        0.,
        SHOT_BOX_SIZE,
        SHOT_LIFE,
        0.,
    )
}
