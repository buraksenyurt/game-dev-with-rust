use crate::constant::*;
use crate::fermat::angle_to_vec;
use crate::sprite::Sprite;
use crate::sprite_type::SpriteType;
use ggez::mint::Point2;
use oorandom::Rand32;
use std::f32::consts::PI;
type Vector2 = glam::Vec2;

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

// Verilen rastgele sayı üretici koşullarına bağlı olarak istenen sayıda farklı açılardan gelen
// kayaları oluşturmak için kullanılan fonksiyon.

pub fn create_random_rocks(
    randomizer: &mut Rand32,
    count: u8,
    exclusion: Point2<f32>,
    min_radius: f32,
    max_radius: f32,
) -> Vec<Sprite> {
    (0..count)
        .map(|_| {
            // Bir kaya üret
            let mut rock = create_sprite(SpriteType::Rock);
            // rastgele bir açı değeri al
            let r_angle = randomizer.rand_float() * 2. * PI;
            // rastgele bir mesafe değeri al (oyuncuya belli bir mesafede olmasını istiyoruz)
            let r_distance = randomizer.rand_float() * (max_radius - min_radius) + min_radius;
            // açıyı birim vektöre çevir
            let mut v = angle_to_vec(r_angle);
            // birim vektörü mesafe kadar ötele
            v.x *= r_distance;
            v.y *= r_distance;
            // kayanın yeni pozisyonunu yeni birim vektör değerlerine göre ayarla
            rock.position.x = exclusion.x + v.x;
            rock.position.y = exclusion.y + v.y;
            rock
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::sprite_builder::{create_random_rocks, create_sprite};
    use crate::sprite_type::SpriteType;
    use ggez::mint::Point2;
    use oorandom::Rand32;

    #[test]
    pub fn should_created_sprite_type_correct() {
        let rock = create_sprite(SpriteType::Rock);
        assert_eq!(rock.kind, SpriteType::Rock);

        let player = create_sprite(SpriteType::Player);
        assert_eq!(player.kind, SpriteType::Player);

        let shot = create_sprite(SpriteType::Shot);
        assert_eq!(shot.kind, SpriteType::Shot);
    }

    #[test]
    pub fn should_create_random_ten_rocks() {
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Randomizer oluşturulurken hata!");
        let mut randomizer = Rand32::new(u64::from_ne_bytes(seed));
        let rocks = create_random_rocks(&mut randomizer, 10, Point2 { x: 0., y: 0. }, 100., 360.);
        assert!(rocks.len() == 10);
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
