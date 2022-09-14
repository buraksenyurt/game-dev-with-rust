// Oyundaki nesnelerin açısal hareketleri vs düşünüldüğünde bazı yardımcı fonksiyonar gerekiyor.
// Genellikle matematiksel operasyonların kullanıldığı fonksiyonlar olarak düşünebilir.
// Oyunda özellikle açıya göre birim vektörün bir başka deyişle vektör yönünün hesaplandığı yerler var.

type Vector2 = glam::Vec2;
use oorandom::Rand32;
use std::f32::consts::PI;

// Verilen açıyı alıp birim vektöre çeviren fonksiyon (vektörün yönünü buluyoruz)
// Asıl ilgilendiğimiz alan vektörün büyüklüğünden ziyade yönü.
// Mesela kayaya rastgele bir yön vermek istediğimizde bu çevrimler çok işe yarayacaktır.
pub fn angle_to_vec(angle: f32) -> Vector2 {
    Vector2 {
        x: angle.sin(),
        y: angle.cos(),
    }
}

// Maksimum büyüklüğe göre rastgele üretilen bir açının birim vektör karşılığını üreten fonksiyon.
pub fn magnitude_to_vector(randomizer: &mut Rand32, max_magnitude: f32) -> Vector2 {
    let angle = randomizer.rand_float() * 2. * PI;
    let magnitude = randomizer.rand_float() - max_magnitude;
    let v2 = angle_to_vec(angle);
    Vector2 {
        x: v2.x * magnitude,
        y: v2.y * magnitude,
    }
}

#[cfg(test)]
mod test {
    use crate::fermat::angle_to_vec;

    #[test]
    pub fn should_angle_convert_to_vector() {
        let v2 = angle_to_vec(45.);
        assert_eq!(v2.x, 0.8509035);
        assert_eq!(v2.y, 0.52532196);

        let v2 = angle_to_vec(90.);
        assert_eq!(v2.x, 0.89399666);
        assert_eq!(v2.y, -0.44807363);

        let v2 = angle_to_vec(120.);
        assert_eq!(v2.x, 0.58061117);
        assert_eq!(v2.y, 0.81418097);

        let v2 = angle_to_vec(360.);
        assert_eq!(v2.x, 0.9589157);
        assert_eq!(v2.y, -0.28369108);

        let v2 = angle_to_vec(0.);
        assert_eq!(v2.x, 0.);
        assert_eq!(v2.y, 1.);
    }
}
