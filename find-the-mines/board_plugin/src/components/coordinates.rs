use std::ops::{Add, Sub};

// Component olarak kullanacağımız veri yapısı tasarlanıyor.
// x ve y koordinat bilgilerini tutmak üzere tasarlandı.
// Inspector tarafından da debug modda görülebilecek
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Clone, Copy)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

// Koordinat bilgilerini birbirlerine ekleyebilmek veya çıkarabilmek için
// Add ve Sub trait'leri uyarlanmakta
impl Add for Coordinates {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // saturating_sub ile negatif değer oluşması engelleniyor
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

// tile_map.rs içinde kullanılan safe_square_at fonksiyonunda
// bir toplama işlemi var. Coordinates veri yapısına SQUARE_COORDINATES tuple'ını ekliyor.
// Bu toplamanın nasıl yapılacağını çalışma zamanına öğretmek lazım.
// Aşağıdaki fonksiyon bu amaçlar kullanılıyor.
impl Add<(i8, i8)> for Coordinates {
    type Output = Self;

    fn add(self, (x, y): (i8, i8)) -> Self::Output {
        let x = ((self.x as i16) + x as i16) as u16;
        let y = ((self.y as i16) + y as i16) as u16;
        Self { x, y }
    }
}
