// Oyunda gidilebilecek mekanları temsil eden veri yapısıdır.
// Mesela oyuncu uzay mekiğine, çocukluğundaki göl evine, sürücü odasına, mahzene vb yerlere gidebilir.
pub struct Location {
    pub name: String,
    // Mekan ile ilgili daha güzel metinsel bilgi vermek için kullanılır.
    pub description: String,
}

impl Location {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
}
