use crate::constant::*;
use crate::sprite::Sprite;
use crate::sprite_type::SpriteType;
use ggez::mint::{Point2, Vector2};

// Oyundaki nesneleri oluşturmak için bir yardımcı fonksiyon.
// Parametre olarak türü alıp ona göre bir constructor çalıştırıyor.

pub fn create_sprite(kind: SpriteType) -> Sprite {
    match kind {
        SpriteType::Player => Sprite::new(
            SpriteType::Player,
            Point2 { x: 0., y: 0. },
            Vector2 { x: 0., y: 0. },
            0.,
            PLAYER_BOX_SIZE,
            PLAYER_LIFE,
            0.,
        ),
        SpriteType::Rock => Sprite::new(
            SpriteType::Rock,
            Point2 { x: 0., y: 0. },
            Vector2 { x: 0., y: 0. },
            0.,
            ROCK_BOX_SIZE,
            ROCK_LIFE,
            0.,
        ),
        SpriteType::Shot => Sprite::new(
            SpriteType::Shot,
            Point2 { x: 0., y: 0. },
            Vector2 { x: 0., y: 0. },
            0.,
            SHOT_BOX_SIZE,
            SHOT_LIFE,
            0.,
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::sprite_builder::create_sprite;
    use crate::sprite_type::SpriteType;

    #[test]
    pub fn should_created_sprite_type_correct() {
        let rock = create_sprite(SpriteType::Rock);
        assert_eq!(rock.kind, SpriteType::Rock);

        let player = create_sprite(SpriteType::Player);
        assert_eq!(player.kind, SpriteType::Player);

        let shot = create_sprite(SpriteType::Shot);
        assert_eq!(shot.kind, SpriteType::Shot);
    }
}

// region Sprite oluşturmak için kullanılan önceki versiyon fonksiyonları

// pub fn create_player() -> Sprite {
//     Sprite::new(
//         SpriteType::Player,
//         Point2 { x: 0., y: 0. },
//         Vector2 { x: 0., y: 0. },
//         0.,
//         PLAYER_BOX_SIZE,
//         PLAYER_LIFE,
//         0.,
//     )
// }
//
// pub fn create_rock() -> Sprite {
//     Sprite::new(
//         SpriteType::Rock,
//         Point2 { x: 0., y: 0. },
//         Vector2 { x: 0., y: 0. },
//         0.,
//         ROCK_BOX_SIZE,
//         ROCK_LIFE,
//         0.,
//     )
// }
//
// pub fn create_shot() -> Sprite {
//     Sprite::new(
//         SpriteType::Shot,
//         Point2 { x: 0., y: 0. },
//         Vector2 { x: 0., y: 0. },
//         0.,
//         SHOT_BOX_SIZE,
//         SHOT_LIFE,
//         0.,
//     )
// }

// endregion
