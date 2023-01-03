use std::ops::{Add, Sub};

// Component olarak kullanacağımız veri yapısı tasarlanıyor.
// x ve y koordinat bilgilerini tutmak üzere tasarlandı.
// Inspector tarafından da debug modda görülebilecek
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default)]
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
