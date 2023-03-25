use crate::location::Location;

// Oyundaki mekanlar, alet edavatlar ve hatta aktörler şimdilik Object olarak ifade ediliyor
pub struct Object {
    pub name: String,
    pub description: String,
    // Aslında fiziki sabit mekanlar için lokasyon kullanmayacağız ama alet edavatın
    // ve hatta karakterlerin lokasyon bilgisi gerekli. Bu nedenle opsiyonel.
    pub location: Option<Location>,
}

impl Object {
    pub fn new(name: String, description: String, location: Option<Location>) -> Self {
        Self {
            name,
            description,
            location,
        }
    }
}
