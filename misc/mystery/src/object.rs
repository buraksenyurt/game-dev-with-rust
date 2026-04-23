use crate::location::Location;
use crate::noun::Noun;

// Oyundaki mekanlar, alet edavatlar ve hatta aktörler şimdilik Object olarak ifade ediliyor
pub struct Object {
    pub noun: Noun,
    pub description: String,
    // Aslında fiziki sabit mekanlar için lokasyon kullanmayacağız ama alet edavatın
    // ve hatta karakterlerin lokasyon bilgisi gerekli. Bu nedenle opsiyonel.
    pub location: Option<Location>,
}

impl Object {
    pub fn new(noun: Noun, description: String, location: Option<Location>) -> Self {
        Self {
            noun,
            description,
            location,
        }
    }
}
